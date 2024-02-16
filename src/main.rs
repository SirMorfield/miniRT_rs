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
use minifb::{Key, Window, WindowOptions};
use std::sync::Arc;
use util::fps_to_duration;

//https://crates.io/crates/minifb
fn main() {
    let resolution = get_resolution();
    resolution.print();
    let mut renderer = MultiThreadedRenderer::new(resolution);
    let scene = Arc::new(get_scene().unwrap());
    let mut window = Window::new(
        "Test - ESC to exit",
        resolution.width.get(),
        resolution.height.get(),
        WindowOptions::default(),
    )
    .unwrap();
    window.limit_update_rate(Some(fps_to_duration(30)));
    window
        .update_with_buffer(
            &vec![0; resolution.width.get() * resolution.height.get()],
            resolution.width.get(),
            resolution.height.get(),
        )
        .unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        renderer.render(&scene);
        let fb = renderer.frame_buffer.lock().unwrap();
        // if window.is_key_down(Key::Space) {
        //     renderer.scene.camera.pos.y += 0.1;
        // }
        window
            .update_with_buffer(
                &fb.buffer(),
                resolution.width.get(),
                resolution.height.get(),
            )
            .unwrap();
    }
}
