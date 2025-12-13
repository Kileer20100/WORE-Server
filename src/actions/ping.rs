

use tokio_tungstenite::tungstenite::Message;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use futures::stream::SplitSink;

use futures::SinkExt;
use serde_json::{self, Value};

use serde::Deserialize;
#[derive(Deserialize)]
struct PayloadPing{
    ping_message: i32,
}


pub async fn handle_ping(write: &mut SplitSink<WebSocketStream<TcpStream>, Message>, text: Value){
    
    let payload_ping: PayloadPing = serde_json::from_value(text).unwrap();

    write.send(payload_ping.ping_message.to_string().into()).await.unwrap();
}