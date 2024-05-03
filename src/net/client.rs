use crate::frame_buffer::PixelProvider;
use crate::net::NetCommand;
use crate::scene_readers::Scene;
use crate::util::{PixelReq, PixelReqBuffer};
use std::io::Error;
use std::net::TcpStream;
use NetCommand::RenderPixel;

use super::socket::NetSocket;

pub struct NetClient {
    scene: Option<Scene>,
    reader: NetSocket,
}

impl NetClient {
    pub fn new(server_address: &str) -> Result<Self, Error> {
        let stream = TcpStream::connect(&server_address)?;
        println!("Connecting to server at {}", server_address);
        Ok(Self {
            scene: None,
            reader: NetSocket::new(stream),
        })
    }

    pub fn read_next_pixel(&mut self) -> PixelReqBuffer {
        let message: Vec<u8> = self.reader.read().unwrap();
        let cmd = serde_cbor::from_slice(&message).unwrap();

        match cmd {
            NetCommand::ReadScene(scene) => {
                println!("Received scene");
                self.scene = Some(scene)
            }
            RenderPixel(pixel_req) => {
                println!("Received pixel request");
                if self.scene.is_none() {
                    println!("WARN: received RenderPixel command without scene loaded, skipping");
                } else {
                    return pixel_req;
                }
            }
            _ => {
                eprintln!("Received unknown command: {:?}", cmd);
            }
        }
        return self.read_next_pixel();
    }
}

impl Iterator for NetClient {
    type Item = PixelReqBuffer;

    fn next(&mut self) -> Option<PixelReqBuffer> {
        Some(self.read_next_pixel())
    }
}
