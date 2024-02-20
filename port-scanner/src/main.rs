use port_scanner::{check_port, PortResult};
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let host: &str = "127.0.0.1";
    for p in 1..65536 {
        let query: String = [&host, ":", &p.to_string()].join("");
        tokio::spawn( async move {let _ = check_port(&query).await;} );
    }
    Ok(())
}