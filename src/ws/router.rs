

use serde_json::Value;
use tokio::net::TcpListener;
use tokio::net::TcpStream; 
use tokio_tungstenite::accept_async;
use futures::{StreamExt, SinkExt};

use serde::{Serialize, Deserialize};
use serde_json;


#[derive(Serialize, Deserialize, Debug)]
struct Msg {
    action: String,
    payload: Value,
}

pub async fn routing_json(text:String){

    let parsed: Msg = serde_json::from_str(&text).unwrap();
    
    println!("{:?}", parsed.action);
    println!("{:?}", parsed.payload);

    if parsed.action == "ping" {
        println!("ping this");
    }
}