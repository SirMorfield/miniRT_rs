extern crate bmp;
extern crate num_integer;

use std::sync::Arc;
use std::sync::RwLock;

use crate::frame_buffer::PixelProvider;
use crate::init::Mode;
use crate::net::NetClient;
use crate::net::NetServer;
use crate::renderer::render_multithreaded;
use init::get_resolution;
use init::get_scene;
use init::Argv;

mod camera;
mod frame_buffer;
mod helpers;
mod init;
mod light;
mod net;
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

fn main() {
    let argv = Argv::new();
    let resolution = get_resolution();
    let mut fb = frame_buffer::FrameBuffer::new(&resolution).unwrap();
    let scene = get_scene(&argv.input_file).unwrap();

    match argv.mode {
        Mode::NetServer => {
            let mut server = NetServer::new(&argv.address.unwrap(), scene, &resolution);
            server.start()
        }
        Mode::NetClient => {
            let scene = Arc::new(RwLock::new(scene));
            let pixel_provider = NetClient::new(&argv.address.unwrap()).unwrap();
            let mut pixels = render_multithreaded(scene, &resolution, pixel_provider);
            fb.set_pixel_from_iterator(&mut pixels);
        }
        Mode::ToFile => {
            let pixel_provider = PixelProvider::new(&resolution);
            let scene = Arc::new(RwLock::new(scene));
            let mut pixels = render_multithreaded(scene, &resolution, pixel_provider);
            fb.set_pixel_from_iterator(&mut pixels);
            fb.save_as_bmp(&argv.output_file.unwrap()).unwrap();
        }
        Mode::Window => {
            // loop_until_closed(&mut renderer, scene, resolution);
        }
    }
}
