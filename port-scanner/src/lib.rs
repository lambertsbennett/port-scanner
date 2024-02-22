use tokio::net::TcpStream;

pub async fn check_port(query: &str) {
    if let Ok(_stream) = TcpStream::connect(query).await {
        println!("{query} is open!");
    } else {}
}