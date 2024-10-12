#![allow(dead_code)]

extern crate bmp;
extern crate num_integer;

use crate::frame_buffer::PixelProvider;
use crate::init::Mode;
use crate::net::NetClient;
use crate::net::NetServer;
use crate::renderer::render_multithreaded;
use crate::util::ExitOnError;
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

    match argv.mode {
        Mode::NetServer => {
            let resolution = get_resolution();
            let scene = get_scene(&argv.input_file).unwrap();
            let mut server = NetServer::new(&argv.address.unwrap(), scene, &resolution);
            server.start()
        }
        Mode::NetClient => net_client(&argv),
        Mode::ToFile => {
            let resolution = get_resolution();
            let scene = get_scene(&argv.input_file).unwrap();
            let mut fb = frame_buffer::FrameBuffer::new(&resolution).unwrap();
            let pixel_provider = PixelProvider::new(&resolution);
            // let scene = Arc::new(RwLock::new(scene));
            let mut pixels = render_multithreaded(&scene, &resolution, pixel_provider);
            fb.set_pixel_from_iterator(&mut pixels);
            fb.save_as_bmp(&argv.output_file.unwrap()).unwrap();
        }
        Mode::Window => {}
    }
}

fn net_client(argv: &Argv) {
    let mut pixel_provider = NetClient::new(&argv.address.as_ref().unwrap()).exit_with("Failed to connect to server");
    let resolution = get_resolution();
    let mut rendered_pixel_blocks = 0;

    loop {
        let pixel_requests = pixel_provider.read_next_pixel();
        if pixel_requests.into_iter().all(|x| x.is_none()) {
            pixel_provider.disconnect();
            println!("No more pixels to render, disconnected from server");
            break;
        }
        // let pixel_requests = split(&pixel_requests, threads());
        let pixel_requests = Some(pixel_requests).into_iter();

        let scene = &pixel_provider.scene.clone().unwrap();
        let pixel_bufs = render_multithreaded(scene, &resolution, pixel_requests);

        for pixel_buf in pixel_bufs {
            pixel_provider.send_pixel(pixel_buf);
            rendered_pixel_blocks += 1;
        }
        println!("Rendered {} pixel bufs", rendered_pixel_blocks)
    }
}
