extern crate bmp;
extern crate num_integer;

use std::sync::Arc;
use std::sync::RwLock;

use init::Argv;
use init::get_resolution;
use init::get_scene;
use window::loop_until_closed;
use crate::frame_buffer::PixelProvider;
use crate::init::Mode;
use crate::net::NetClient;
use crate::net::NetServer;
use crate::random_iterator::RandomIterator;
use crate::renderer::render_multithreaded;

mod camera;
mod frame_buffer;
mod helpers;
mod init;
mod light;
mod num;
mod octree;
mod progress_logger;
mod random_iterator;
mod renderer;
mod resolution;
mod scene_readers;
mod triangle;
mod util;
mod vector;
mod window;
mod net;

fn main() {
    let argv = Argv::new();
    let resolution = get_resolution();
    let mut fb = frame_buffer::FrameBuffer::new(&resolution).unwrap();
    let scene = Arc::new(RwLock::new(get_scene(&argv.input_file).unwrap()));

    match argv.mode {
        Mode::NetServer => {
            let mut server = NetServer::new(&argv.address.unwrap(), scene, &resolution);
            server.start()
        }
        Mode::NetClient => {
            let mut client = NetClient::new(&argv.address.unwrap()).unwrap();
            client.start().unwrap();
        }
        Mode::ToFile => {
            let pixel_provider = PixelProvider::new(&resolution);
            let pixels = render_multithreaded::<1000>(scene, &resolution, pixel_provider);
            for pixel_block in pixels {
                for pixel in pixel_block {
                    if let Some((x, y, color)) = pixel {
                        fb.set_pixel(x, y, color);
                    }
                }
            }
            fb.save_as_bmp(&argv.output_file.unwrap()).unwrap();
        }
        Mode::Window => {
            // loop_until_closed(&mut renderer, scene, resolution);
        }
    }
}
