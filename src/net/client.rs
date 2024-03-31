use std::net::TcpStream;
use std::io::Error;
use crate::net::NetCommand;
use crate::scene_readers::Scene;

use super::socket::NetSocket;

pub struct NetClient {
    server_address: String,
    scene: Option<Scene>,
}

impl NetClient {
    pub fn new(server_address: &str) -> Result<Self, Error> {
        Ok(Self {
            server_address: server_address.to_string(),
            scene: None
        })
    }

    pub fn start(&mut self) -> Result<(), Error> {
        println!("Connecting to server at {}", self.server_address);
        let stream = TcpStream::connect(&self.server_address)?;
        let mut reader = NetSocket::new(stream);

        loop {
            let message: Vec<u8> = reader.read()?;
            let cmd = serde_cbor::from_slice(&message).unwrap();
            println!("Received command: {:?}", cmd);
            match cmd {
                NetCommand::ReadScene(scene) => self.scene = Some(scene),
                NetCommand::RenderPixel(coordinate) => {
                    println!("Received render request for {:?}", coordinate);
                }
                _ => {
                    eprintln!("Received unknown command: {:?}", cmd);
                }
            }
        }
    }
}
