
mod toml;
mod audit;
mod ws;
mod actions;
mod error;

use tokio::net::TcpListener;
use audit::audit::audit;
use ws::connection::handle_connection;


#[tokio::main]
async fn main() -> tokio::io::Result<()>{
    audit();

    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr).await?;
    println!("Server start:{}",addr);


    // New theard user WS connect
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));        
    }

    Ok(())
}

