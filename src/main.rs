
mod toml;
mod audit;
mod ws;


use tokio::net::TcpListener;
use audit::audit::audit;
use ws::connection::hendl_connection;


#[tokio::main]
async fn main() -> tokio::io::Result<()>{
    audit();

    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr).await?;
    println!("Server start:{}",addr);


    // New theard user WS connect
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(hendl_connection(stream));        
    }

    Ok(())
}

