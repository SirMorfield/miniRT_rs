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
use std::sync::RwLock;
use util::fps_to_duration;

//https://crates.io/crates/minifb
fn main() {
    let resolution = get_resolution();
    resolution.print();
    let mut renderer = MultiThreadedRenderer::new(resolution);
    let scene = get_scene().unwrap();
    let scene_arc = Arc::new(RwLock::new(scene));

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
        renderer.render(&scene_arc, false);
        let fb = renderer.frame_buffer.lock().unwrap();
        for key in window.get_keys_pressed(minifb::KeyRepeat::Yes) {
            scene_arc
                .write()
                .unwrap()
                .camera
                .move_from_keyboard(key, 10.0);
        }

        window
            .update_with_buffer(
                &fb.buffer(),
                resolution.width.get(),
                resolution.height.get(),
            )
            .unwrap();
    }
}
