use clap::{Parser, Subcommand};
use mio::net::TcpStream;
use std::io::Write;
use std::net::SocketAddr;
use std::str;
use std::time::Duration;

const SERVER_ADDR: &str = "127.0.0.1";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// port of server to connect to
    #[arg(short, long, value_name = "PORT")]
    port: u32,
}

fn main() {
    let cli = Cli::parse();
    //connect to the server
    let server_addr: SocketAddr = format!("{}:{}", SERVER_ADDR, cli.port)
        .parse()
        .expect("Invalid server address");
    println!("Connect to {server_addr}");

    let mut stream = TcpStream::connect(server_addr).expect("Failed to connect to server");

    loop {
        std::thread::sleep(Duration::from_millis(500));
        println!("Ping");
        stream.write_all(b"ping").expect("Failed to write");
    }
}
