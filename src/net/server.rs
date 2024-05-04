use crate::frame_buffer::{FrameBuffer, PixelProvider};
use crate::net::{NetCommand, NetResponse, NetSocket};
use crate::resolution::Resolution;
use crate::scene_readers::Scene;
use std::io::ErrorKind;
use std::net::TcpListener;
use std::path::Path;
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
                if *state == SocketState::Initiated {
                    let response = socket.read();
                    match response {
                        Ok(response) => {
                            let response: NetResponse = serde_cbor::from_slice(&response).unwrap();

                            match response {
                                NetResponse::RenderPixel(buffer) => self.frame_buffer.set_pixel_from_buffer(&buffer),
                            }
                            println!("Progress: {}%", self.frame_buffer.progress().get());
                        }
                        Err(e) => {
                            if e.kind() != ErrorKind::WouldBlock {
                                eprintln!("Client error: {}", e);
                                *state = SocketState::Disconnected;
                            }
                        }
                    }
                }

                let cmd = match state {
                    SocketState::Uninitialized => NetCommand::ReadScene(self.scene.clone()),
                    SocketState::Initiated => {
                        let coordinate = self.pixel_stream.get_coordinates();
                        if self.frame_buffer.is_complete() {
                            println!("All pixels rendered, resetting");
                            self.frame_buffer.save_as_bmp(Path::new("output.bmp")).unwrap();
                            self.pixel_stream.reset();
                            return;
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

    fn read_response(
        &self,
        response: &Result<Vec<u8>, std::io::Error>,
        status: &mut SocketState,
    ) -> Option<NetResponse> {
        match response {
            Ok(response) => {
                let response: NetResponse = serde_cbor::from_slice(&response).unwrap();
                return Some(response);
            }
            Err(e) => {
                if e.kind() != ErrorKind::WouldBlock {
                    eprintln!("Client error: {}", e);
                    *status = SocketState::Disconnected;
                }
                return None;
            }
        }
    }
}
