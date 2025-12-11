
use futures::sink::Send;
use serde_json::Value;
use tokio::net::TcpListener;
use tokio::net::TcpStream; 
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};

use serde::{Serialize, Deserialize};
use serde_json;
use crate::ws::router::routing_json;



pub async fn hendl_connection(stream: TcpStream){
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("WebSocket handshake error: {}", e);
            return;
        }
        
    };
    println!("New WebSocket connection");

    let(mut write , mut read) = ws_stream.split();
    while let Some(msg) = read.next().await {
        match msg {

            Ok(msg) => match msg {
                
                tokio_tungstenite::tungstenite::Message::Text(text) =>{
                        routing_json(text).await;
                        write.send("dd".into()).await.unwrap();
                    }

                tokio_tungstenite::tungstenite::Message::Close(_) => break,
                    _ => {}
            
            }
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }

    println!("Connection closed");

}