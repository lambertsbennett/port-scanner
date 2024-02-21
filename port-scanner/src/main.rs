use port_scanner::{check_port, ping_host};
use std::fmt;
use std::env;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct ArgParseError {
    pub message: String,
}

impl  fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}", self.message)
    }
}

impl Error for ArgParseError {}

#[tokio::main]
async fn main() -> Result<()> {

    let target = env::args().nth(1).ok_or(ArgParseError{message: String::from("No Arguments Provided!")})?;
    let ping_target = target.clone();
    let handle = tokio::spawn( async move { let _ = ping_host(&ping_target).await; } );
    let _out = handle.await?;
    for p in 1..65536 {
        let query: String = [&target, ":", &p.to_string()].join("");
        tokio::spawn( async move { check_port(&query).await; } );
    }
    Ok(())
}