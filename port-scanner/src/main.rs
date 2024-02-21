use port_scanner::check_port;
use std::fmt;
use std::env;
use std::error::Error;

#[derive(Debug)]
pub struct ArgParseError {
    pub message: String,
}

impl  fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ArgParseError {}

#[tokio::main]
async fn main() -> Result<(), ArgParseError> {

    let target = env::args().nth(1);
    match target {
        Some(target) =>  {
            for p in 1..65536 {
                let query: String = [&target, ":", &p.to_string()].join("");
                tokio::spawn( async move {let _ = check_port(&query).await;} );
            }
            Ok(())
        },
        None => Err(ArgParseError{message: String::from("No Arguments Provided!")})
    }
}