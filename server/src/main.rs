use clap::{Parser, Subcommand};
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::net::SocketAddr;

// the server should print out message from client and when they connect and disconnect
const SERVER: Token = Token(0);

fn main() {
    // Create a poll instance.
    let mut poll = Poll::new().unwrap();
    //Create storage for events

    let mut events = Events::with_capacity(128);

    //set up the server socket
    let addr = "127.0.0.1:13265".parse().unwrap();
    let mut server = TcpListener::bind(addr).unwrap();

    // start listening for incoming connections
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)
        .unwrap();
    // create client_id for client token generation and store client msg and addr in hashmap
    let mut client_id = 1;
    let mut clients: HashMap<Token, (TcpStream, SocketAddr)> = HashMap::new();
    let mut buf = [0; 1024];

    loop {
        // Poll Mio for events, blocking until we get an event
        poll.poll(&mut events, None).unwrap();

        //process each event, event is tell if there is things in it, it could be client or server token, from token,
        // we can get stream and addr from the map
        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let token = Token(client_id);
                    client_id += 1;
                    let (mut stream, addr) = server.accept().unwrap();
                    poll.registry()
                        .register(&mut stream, token, Interest::READABLE)
                        .unwrap();
                    clients.insert(token, (stream, addr));
                }
                token => {
                    if !event.is_readable() {
                        continue;
                    }
                    let (stream, addr) = clients.get_mut(&token).unwrap();
                    let msg_len = stream.read(&mut buf).unwrap();
                    if msg_len == 0 {
                        poll.registry().deregister(stream).unwrap();
                        clients.remove(&token);
                        println!("client disconnect")
                    } else {
                        match std::str::from_utf8(&buf) {
                            Ok(utf8_str) => println!("{}:{}", addr, utf8_str),
                            Err(_) => println!(),
                        }
                    }
                }
            }
        }
    }
}
