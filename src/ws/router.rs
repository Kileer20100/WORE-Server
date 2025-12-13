

use serde_json::Value;

use futures::SinkExt;

use tokio_tungstenite::tungstenite::Message;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use futures::stream::SplitSink;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::actions::ping::handle_ping;

#[derive(Serialize, Deserialize, Debug)]
struct IncomingMessage {
    action: String,
    payload: Value,
}

pub async fn routing_json(
    text: &String,
    writer: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
){

    let parsed: IncomingMessage = serde_json::from_str(&text).unwrap();
    
    match parsed.action.as_str() {
        "ping" => handle_ping(writer, parsed.payload).await,
        _=> writer.send("Error 000".into()).await.unwrap(),
    }
}