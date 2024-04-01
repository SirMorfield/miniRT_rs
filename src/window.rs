use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use std::time;
use std::time::Duration;

use minifb::Key;

use crate::resolution::Resolution;
use crate::scene_readers;

pub fn loop_until_closed(
    scene: Arc<RwLock<scene_readers::Scene>>,
    resolution: Resolution,
) {
    let mut window = get_window(&resolution);
    let mut pressed = true;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for key in window.get_keys() {
            pressed = pressed || scene.write().unwrap().camera.keyboard(&key);
            if key == Key::E {
                pressed = true;
                let path = PathBuf::from("scene.cbor");
                let file_size = scene.read().unwrap().save_to_file(&path).unwrap();
                println!("Scene saved to {:?} ({})", path, file_size);
            }
        }
        // if pressed {
        //     renderer.render(&scene, false);
        //     let fb = renderer.frame_buffer.lock().unwrap();
        //     window
        //         .update_with_buffer(&fb.buffer(), resolution.width.get(), resolution.height.get())
        //         .unwrap();
        // } else {
        //     window.update();
        //     std::thread::sleep(Duration::from_micros(16600));
        // }
        pressed = false;
    }
}

fn get_window(resolution: &Resolution) -> minifb::Window {
    let mut window = minifb::Window::new(
        "Test - ESC to exit",
        resolution.width.get(),
        resolution.height.get(),
        minifb::WindowOptions::default(),
    )
    .unwrap();
    window
        .update_with_buffer(
            &vec![0; resolution.width.get() * resolution.height.get()],
            resolution.width.get(),
            resolution.height.get(),
        )
        .unwrap();
    window.limit_update_rate(Some(time::Duration::from_micros(16600)));
    window
}
