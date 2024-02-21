use tokio::net::TcpStream;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct PingError {
    pub message: String,
}

impl fmt::Display for PingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PingError {}

pub async fn ping_host(host: &str) -> Result<(), Box<dyn Error>>{
    let stream = TcpStream::connect(host).await?;
    match stream.try_write(&[1]) {
        Ok(_n) => {},
        Err(_) => return Err(Box::new(PingError{message: String::from("Error writing to TCPStream!")}))
    }
    match stream.try_read(&mut [0; 128]){
        Ok(_n) => {},
        Err(_) => return Err(Box::new(PingError{message: String::from("Error reading from TCPStream!")}))
    }
    Ok(())
}

pub async fn check_port(query: &str) {
    if let Ok(_stream) = TcpStream::connect(query).await {
        println!("{query} is open!");
    } else {}
}