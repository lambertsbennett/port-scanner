use tokio::net::TcpStream;
use std::error::Error;

#[derive(Debug)]
pub enum PortResult {
    Open,
    Closed
}

pub async fn check_port(query: &str) -> Result<PortResult, Box<dyn Error>>{
    if let Ok(_stream) = TcpStream::connect(query).await {
        println!("{query} is open!");
        Ok(PortResult::Open)
    } else {
        Ok(PortResult::Closed)
    }
}