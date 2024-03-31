use std::net::TcpStream;
use std::io::Error;

use super::socket::NetSocket;

pub struct NetClient {
    server_address: String,
}

impl NetClient {
    pub fn new(server_address: &str) -> Result<Self, Error> {
        Ok(Self {
            server_address: server_address.to_string(),
        })
    }

    pub fn start(&mut self) -> Result<(), Error> {
        println!("Connecting to server at {}", self.server_address);
        let stream = TcpStream::connect(&self.server_address)?;
        let mut reader = NetSocket::new(stream);

        loop {
            let message: Vec<u8> = reader.read()?;
            println!(
                "from server: {} {:?}",
                message.len(),
                // String::from_utf8_lossy(message.as_ref())
                message
            );
        }
    }
}
