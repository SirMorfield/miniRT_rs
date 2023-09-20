mod helpers;
use helpers::*;
mod resolution;
use resolution::Resolution;
mod frame_buffer;
use frame_buffer::FrameBuffer;
mod foreign_function_interfaces;
mod vector;
use std::{path::PathBuf, time};
extern crate bmp;
mod scene;
use scene::Scene;
use std::path::Path;
mod renderer;
mod triangle;
use renderer::Renderer;
mod camera;
mod light;
mod util;

fn get_rt_file() -> Option<PathBuf> {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        println!("Usage: {} <scene.rt>", argv.get(0).unwrap());
        return None;
    }
    Path::new(argv.get(1).unwrap()).canonicalize().ok()
}
fn main() {
    let scene_path = get_rt_file().unwrap();
    let scene = Scene::new(scene_path.as_path()).unwrap();
    let resolution = Resolution::new(500, 500).unwrap();
    let mut frame_buffer = FrameBuffer::new(resolution).unwrap();
    let renderer = Renderer::new(resolution);

    let start = time::Instant::now();
    loop {
        match frame_buffer.get_coordinate() {
            None => break,
            Some((x, y)) => {
                let color = renderer.render(&scene, &scene.camera, x as f32, y as f32);
                frame_buffer.set_pixel(x, y, color);
                print!("{}%     \r", frame_buffer.progress() * 100.0);
            }
        }
    }
    let duration = start.elapsed();
    println!("Scene rendered in: {}", duration.as_formatted_str());

    let path = Path::new("output.bmp");
    frame_buffer.save_as_bmp(path).unwrap();
    println!("Saved to: {}", path.display());
}
