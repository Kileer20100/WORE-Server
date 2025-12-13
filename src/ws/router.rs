

use serde_json::Value;

use futures::SinkExt;

use tokio_tungstenite::tungstenite::Message;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use futures::stream::SplitSink;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::actions::ping::handle_ping;
use crate::error::error::ServeError;

#[derive(Serialize, Deserialize, Debug)]
struct IncomingMessage {
    action: String,
    payload: Value,
}

pub async fn routing_json(
    msg: &String,
    writer: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
){

    let parsed: IncomingMessage = match serde_json::from_str(&msg){
        Ok(parsed) => parsed,
        Err(_) => {
        let _ = writer.send(ServeError::ErrorRoutingParsingMessage.to_string().into()).await;
        return;
        }
    };
    
    match parsed.action.as_str() {
        "ping" => handle_ping(writer, parsed.payload).await,
        action=> {
            let _ = writer.send(
                ServeError::ErrorRoutingMessage(action.to_string()).to_string().into()
            ).await;},
    }
}