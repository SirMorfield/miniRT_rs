extern crate bmp;
extern crate num_integer;

use std::sync::Arc;
use std::sync::RwLock;

use crate::frame_buffer::PixelProvider;
use crate::init::Mode;
use crate::net::NetClient;
use crate::net::NetServer;
use crate::renderer::render_multithreaded;
use crate::util::ExitOnError;
use init::get_resolution;
use init::get_scene;
use init::Argv;
use resolution::Resolution;

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
        Mode::NetClient => net_client(&argv, &resolution),
        Mode::ToFile => {
            let pixel_provider = PixelProvider::new(&resolution);
            // let scene = Arc::new(RwLock::new(scene));
            let mut pixels = render_multithreaded(&scene, &resolution, pixel_provider);
            fb.set_pixel_from_iterator(&mut pixels);
            fb.save_as_bmp(&argv.output_file.unwrap()).unwrap();
        }
        Mode::Window => {}
    }
}

fn net_client(argv: &Argv, resolution: &Resolution) {
    let mut pixel_provider = NetClient::new(argv.address.as_ref().unwrap()).exit_with("Failed to connect to server");

    let mut rendered_pixel_blocks = 0;
    loop {
        let pixel_requests = Some(pixel_provider.read_next_pixel());
        let pixel_request_iter = pixel_requests.into_iter();
        let pixels = render_multithreaded(&pixel_provider.scene.clone().unwrap(), &resolution, pixel_request_iter);
        for pixel in pixels {
            pixel_provider.send_pixel(pixel);
            rendered_pixel_blocks += 1;
        }
        println!("Rendered {} pixels", rendered_pixel_blocks)
    }
}
