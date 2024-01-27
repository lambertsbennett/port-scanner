use std::net::TcpStream;
use port_scanner::ThreadPool;

fn main() {
    let host: &str = "127.0.0.1";
    let pool = ThreadPool::new(12);

    for p in 1..65536
    {
        let query: String = [&host, ":", &p.to_string()].join("");
        
        pool.execute(move || {
            check_port(&query);
        });
    }
}

fn check_port(query: &str){
    if let Ok(_stream) = TcpStream::connect(query) {
        println!("{} is open!", query);
    } else {}
}