

use tokio::net::TcpStream; 
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};


use tungstenite::Message;
use crate::ws::router::routing_json;

use crate::error::error::ServeError;

pub async fn hendl_connection(stream: TcpStream){
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("WebSocket handshake error: {}", e);
            return;
        }
        
    };
    println!("New WebSocket connection");

    let(mut writer , mut read) = ws_stream.split();
    
    while let Some(msg) = read.next().await {
        match msg {

            Ok(msg) => match msg {
                
                tokio_tungstenite::tungstenite::Message::Text(text) =>{
                        routing_json(&text, &mut writer).await;
                    }

                tokio_tungstenite::tungstenite::Message::Close(_) => break,
                    _ => {}
            
            }
            Err(e) => {
                let error_message = format!("{}",e);
                let msg: Message = ServeError::ErrorSendMessage(error_message).to_string().into();
                let _ = writer.send(msg).await;
                break;
            }
        }
    }


}