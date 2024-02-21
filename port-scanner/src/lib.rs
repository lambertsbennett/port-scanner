use tokio::net::TcpStream;
use std::error::Error;


pub async fn check_port(query: &str) -> Result<(), Box<dyn Error>>{
    if let Ok(_stream) = TcpStream::connect(query).await {
        println!("{query} is open!");
        Ok(())
    } else {
        Ok(())
    }
}