use std::io::Write;
use std::net::SocketAddr;
use std::str;
use std::time::Duration;

use mio::net::TcpStream;

const SERVER_ADDR: &str = "127.0.0.1:8080";

fn main() {
    //connect to the server
    let server_addr: SocketAddr = SERVER_ADDR.parse().expect("Invalid server address");
    let mut stream = TcpStream::connect(server_addr).expect("Failed to connect to server");

    loop {
        std::thread::sleep(Duration::from_millis(500));
        stream.write_all(b"ping").expect("Failed to write");
    }
}
