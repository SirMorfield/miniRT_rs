use crate::net::NetCommand;
use crate::scene_readers::Scene;
use crate::util::{PixelReqBuffer, PixelResBuffer};
use std::io::Error;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use NetCommand::RenderPixel;

use super::socket::NetSocket;
use super::NetResponse;

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

    pub fn send_pixel(&mut self, pixel_res: PixelResBuffer) {
        println!("Sending pixel response");
        let cmd = NetResponse::RenderPixel(pixel_res);
        let binding = serde_cbor::to_vec(&cmd).unwrap();
        self.reader.write(binding.as_slice()).unwrap();
    }
}
