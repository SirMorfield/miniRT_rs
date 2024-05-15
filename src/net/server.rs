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
        let connections = self.connections.clone();

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("Incoming stream");
                        stream.set_nonblocking(true).unwrap();

                        let mut connections = connections.lock().unwrap();
                        let len = connections.len();
                        connections.push((SocketState::Uninitialized, NetSocket::new(stream, len as u64)));
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
                #[rustfmt::skip]
                handle_socket( state, socket, &mut self.frame_buffer, &mut self.pixel_stream, &self.scene);
                thread::sleep(Duration::from_millis(1));
                if self.frame_buffer.is_complete() {
                    return;
                }
            }
            sockets.retain(|(state, _)| *state != SocketState::Disconnected);
        }
    }
}

fn handle_socket(
    state: &mut SocketState,
    socket: &mut NetSocket,
    frame_buffer: &mut FrameBuffer,
    pixel_stream: &mut PixelProvider,
    scene: &Scene,
) {
    if *state == SocketState::Initiated {
        let response = socket.read();
        match response {
            Ok(response) => {
                let response: NetResponse = serde_cbor::from_slice(&response).unwrap();

                match response {
                    NetResponse::RenderPixel(buffer) => frame_buffer.set_pixel_from_buffer(&buffer),
                }
                println!("Progress: {}%", frame_buffer.progress().get());
            }
            Err(e) => {
                if e.kind() != ErrorKind::WouldBlock {
                    eprintln!("Client error: {}", e);
                    *state = SocketState::Disconnected;
                    return;
                }
            }
        }
    }

    let cmd = match state {
        SocketState::Uninitialized => NetCommand::ReadScene(scene.clone()),
        SocketState::Initiated => {
            let coordinate = pixel_stream.get_coordinates();
            if frame_buffer.is_complete() {
                println!("All pixels rendered, resetting");
                frame_buffer.save_as_bmp(Path::new("output.bmp")).unwrap();
                pixel_stream.reset();
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
}
