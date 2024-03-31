use std::error::Error;
use std::io::ErrorKind;
use std::net::{TcpListener};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;
use crate::frame_buffer::FrameBuffer;
use crate::net::{NetCommand, NetSocket};
use crate::resolution::Resolution;
use crate::scene_readers::Scene;

pub struct NetServer {
    address: String,
    connections: Arc<Mutex<Vec<(bool, NetSocket)>>>,
    scene: Arc<RwLock<Scene>>,
    frame_buffer: FrameBuffer,
}

impl NetServer {
    pub fn new(address: &str, scene: Arc<RwLock<Scene>>, resolution: &Resolution) -> NetServer {
        NetServer {
            address: address.to_string(),
            connections: Arc::new(Mutex::new(Vec::new())),
            scene,
            frame_buffer: FrameBuffer::new(resolution).unwrap(),
        }
    }

    pub fn start(&mut self) {
        println!("Server listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();
        let inner_connections = self.connections.clone();
        thread::spawn(move || for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    stream.set_nonblocking(true).unwrap();

                    inner_connections
                        .lock()
                        .unwrap()
                        .push((true, NetSocket::new(stream)));
                }
                Err(e) => eprintln!("Incoming stream error: {}", e)

            }
        });

        loop {
            let mut sockets = self.connections.lock().unwrap();
            for (alive, socket) in sockets.iter_mut().filter(|(alive, _)| *alive) {
                let coordinate = self.frame_buffer.get_coordinate();
                if coordinate.is_none() {
                    *alive = true;
                    continue;
                }
                let coordinate = coordinate.unwrap();
                let cmd = NetCommand::RenderPixel(coordinate);
                let binding = serde_cbor::to_vec(&cmd).unwrap();

                if let Err(e) = socket.write(binding.as_slice()) {
                    if e.kind() == ErrorKind::WouldBlock {
                        *alive = true;
                    } else {
                        eprintln!("Client error: {}", e);
                        *alive = false;
                    }
                } else {
                    *alive = true;
                }

                thread::sleep(Duration::from_millis(1));
            }
            sockets.retain(|(alive, _)| *alive);
        }
    }
}
