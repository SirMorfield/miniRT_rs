use std::error::Error;
use std::io::ErrorKind;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use crate::net::NetSocket;

pub struct NetServer {
    address: String,
    connections: Arc<Mutex<Vec<(bool, NetSocket)>>>,
}

impl NetServer {
    pub fn new(address: &str) -> NetServer {
        NetServer {
            address: address.to_string(),
            connections: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Server listening on {}", self.address);
        let inner_connections = self.connections.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        stream.set_nonblocking(true).unwrap();

                        let mut connections = inner_connections.lock().unwrap();
                        connections.push((true, NetSocket::new(stream)));
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        });

        loop {
            let mut sockets = self.connections.lock().unwrap();
            for (allive, ref mut socket) in sockets.iter_mut() {
                if !*allive {
                    continue;
                }
                if let Err(e) = socket.write(b"Hello, world!") {
                    if (e.kind() != ErrorKind::WouldBlock) {
                        *allive = false;
                        eprintln!("Client error: {}", e);
                    }
                }
            }
            sockets.retain(|(alive, _)| *alive);
        }
    }
}
