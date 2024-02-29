extern crate bmp;
extern crate num_integer;

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

use crate::threading::MultiThreadedRenderer;
use init::get_resolution;
use init::get_scene;
use init::get_window;
use init::Argv;
use minifb::{Key, Window};
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

fn loop_until_closed(
    window: &mut Window,
    renderer: &mut MultiThreadedRenderer,
    scene: Arc<RwLock<scene_readers::Scene>>,
    res: resolution::Resolution,
) {
    let mut pressed = true;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for key in window.get_keys() {
            scene.write().unwrap().camera.keyboard(key);
            pressed = true;
        }
        if pressed {
            renderer.render(&scene, false);
            let fb = renderer.frame_buffer.lock().unwrap();
            window
                .update_with_buffer(&fb.buffer(), res.width.get(), res.height.get())
                .unwrap();
        } else {
            window.update();
            std::thread::sleep(Duration::from_micros(16600));
        }
        pressed = false;
    }
}

fn main() {
    let argv = Argv::new().unwrap();
    let resolution = get_resolution();
    let mut renderer = MultiThreadedRenderer::new(resolution);
    let scene = Arc::new(RwLock::new(get_scene(&argv.input_file).unwrap()));

    if let Some(output_file) = argv.output_file {
        renderer.render(&scene, false);
        let fb = renderer.frame_buffer.lock().unwrap();
        fb.save_as_bmp(&output_file).unwrap();
    } else {
        let mut window = get_window(&resolution);
        loop_until_closed(&mut window, &mut renderer, scene, resolution);
    }
}
