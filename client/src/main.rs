use clap::Parser;
use log::*;
use mio::net::TcpStream;
use std::io::Write;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Port of server to connect to
    #[arg(short, long, value_name = "PORT")]
    port: u32,

    /// Server IP address to connect to
    #[arg(short, long, value_name = "IP_ADDR", default_value = "127.0.0.1")]
    address: IpAddr,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    //connect to the server
    let server_addr: SocketAddr = format!("{}:{}", cli.address, cli.port)
        .parse()
        .expect("Invalid server address");
    info!("Connect to {server_addr}");

    let mut stream = TcpStream::connect(server_addr).expect("Failed to connect to server");

    loop {
        std::thread::sleep(Duration::from_millis(500));
        info!("Ping");
        stream.write_all(b"ping").expect("Failed to write");
    }
}
