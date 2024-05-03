use crate::frame_buffer::{FrameBuffer, PixelProvider};
use crate::net::{NetCommand, NetSocket};
use crate::resolution::Resolution;
use crate::scene_readers::Scene;
use std::io::ErrorKind;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(PartialEq)]
enum SocketState {
    Uninitialized, // First state after connection
    Initiated,     // When the socket is ready to accept pixel requests
    Disconnected,  // When the socket does not work, and should be disconnected
}

pub struct NetServer {
    address: String,
    connections: Arc<Mutex<Vec<(SocketState, NetSocket)>>>,
    scene: Scene,
    frame_buffer: FrameBuffer,
    pixel_stream: PixelProvider,
}

impl NetServer {
    pub fn new(address: &str, scene: Scene, resolution: &Resolution) -> NetServer {
        NetServer {
            address: address.to_string(),
            connections: Arc::new(Mutex::new(Vec::new())),
            scene,
            frame_buffer: FrameBuffer::new(resolution).unwrap(),
            pixel_stream: PixelProvider::new(resolution),
        }
    }

    pub fn start(&mut self) {
        println!("Server listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();
        let inner_connections = self.connections.clone();
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        stream.set_nonblocking(true).unwrap();

                        inner_connections
                            .lock()
                            .unwrap()
                            .push((SocketState::Uninitialized, NetSocket::new(stream)));
                    }
                    Err(e) => eprintln!("Incoming stream error: {}", e),
                }
            }
        });

        loop {
            let mut sockets = self.connections.lock().unwrap();
            for (state, socket) in sockets
                .iter_mut()
                .filter(|(state, _)| *state != SocketState::Disconnected)
            {
                let cmd = match state {
                    SocketState::Uninitialized => NetCommand::ReadScene(self.scene.clone()),
                    SocketState::Initiated => {
                        let coordinate = self.pixel_stream.get_coordinates();
                        if coordinate.iter().all(|c| c.is_none()) {
                            println!("All pixels rendered, shutting down server");
                            thread::sleep(Duration::from_millis(100));
                            continue;
                        }
                        NetCommand::RenderPixel(coordinate)
                    }
                    _ => unreachable!(),
                };
                let binding = serde_cbor::to_vec(&cmd).unwrap();

                if let Err(e) = socket.write(binding.as_slice()) {
                    if e.kind() != ErrorKind::WouldBlock {
                        eprintln!("Client error: {}", e);
                        *state = SocketState::Disconnected;
                    }
                }

                if *state == SocketState::Uninitialized {
                    *state = SocketState::Initiated;
                }
                thread::sleep(Duration::from_millis(1));
            }
            sockets.retain(|(state, _)| *state != SocketState::Disconnected);
        }
    }
}
