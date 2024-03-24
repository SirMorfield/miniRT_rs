extern crate bmp;
extern crate num_integer;

use std::sync::Arc;
use std::sync::RwLock;

use init::Argv;
use init::get_resolution;
use init::get_scene;
use window::loop_until_closed;

use crate::threading::MultiThreadedRenderer;

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
mod threading;
mod triangle;
mod util;
mod vector;
mod window;

fn main() {
    let argv = Argv::new().unwrap();
    let resolution = get_resolution();
    let mut renderer = MultiThreadedRenderer::new(resolution);
    let scene = Arc::new(RwLock::new(get_scene(&argv.input_file).unwrap()));

    if let Some(output_file) = argv.output_file {
        renderer.render(&scene, true);
        let fb = renderer.frame_buffer.lock().unwrap();
        fb.save_as_bmp(&output_file).unwrap();
    } else {
        loop_until_closed(&mut renderer, scene, resolution);
    }
}
