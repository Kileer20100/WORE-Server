use tokio::net::TcpStream; 
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};

use tungstenite::Message;
use crate::ws::router::routing_json;
use crate::error::error::ServeError;

/// Handles a single WebSocket client connection.
/// 
/// This function manages the entire lifecycle of a WebSocket connection:
/// 1. Performs WebSocket handshake
/// 2. Splits the stream into reader and writer
/// 3. Listens for incoming messages
/// 4. Routes messages to appropriate handlers
/// 5. Handles connection errors and cleanup
/// 
/// # Parameters
/// - `stream`: Raw TCP stream from the client
/// 
/// # Process Flow
/// 1. Upgrade TCP to WebSocket protocol via handshake
/// 2. Split connection for bidirectional communication
/// 3. Enter message processing loop
/// 4. Route text messages, handle control frames (Close, Ping/Pong)
/// 5. Clean up on disconnect or error

pub async fn handle_connection(stream: TcpStream) {
    // Step 1: Perform WebSocket handshake
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            // Handshake failed - log error and terminate connection
            println!("{}", ServeError::WebSocketHandshakeError(e.to_string()));
            return;
        }
    };
    
    println!("New WebSocket connection established");
    
    // Step 2: Split WebSocket stream into separate reader and writer
    // This allows concurrent reading and writing
    let (mut writer, mut reader) = ws_stream.split();
    
    // Step 3: Main message processing loop
    // Continuously read messages until connection closes
    while let Some(msg) = reader.next().await {
        match msg {
            // Successfully received a message
            Ok(msg) => match msg {
                // Text message - route to JSON message handler
                tokio_tungstenite::tungstenite::Message::Text(text) => {
                    routing_json(&text, &mut writer).await;
                }
                
                // Close frame - client requested disconnect
                tokio_tungstenite::tungstenite::Message::Close(_) => {
                    break; // Exit loop and clean up connection
                }
                
                // Ignore other message types (Binary, Ping, Pong, etc.)
                _ => {}
            },
            
            // Error receiving message - send error to client and disconnect
            Err(e) => {
                let error_message = format!("{}", e);
                let msg: Message = ServeError::ErrorSendMessage(error_message).to_string().into();
                let _ = writer.send(msg).await; // Try to notify client
                break; // Terminate connection on read error
            }
        }
    }
    
    // Connection loop ended - implicit cleanup via drop
    // WebSocket and TCP connections will be automatically closed
}