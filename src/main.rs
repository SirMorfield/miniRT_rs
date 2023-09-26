extern crate bmp;
mod camera;
mod foreign_function_interfaces;
mod frame_buffer;
mod helpers;
mod light;
mod progress_logger;
mod renderer;
mod resolution;
mod scene;
mod triangle;
mod util;
mod vector;

use frame_buffer::FrameBuffer;
use progress_logger::ProgressLogger;
use renderer::Renderer;
use resolution::Resolution;
use scene::Scene;
use std::path::Path;
use std::path::PathBuf;
use util::PositiveNonzeroF32;

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
    let mut progress_logger =
        ProgressLogger::new("Rendering", PositiveNonzeroF32::new(0.1).unwrap(), 1);

    loop {
        match frame_buffer.get_coordinate() {
            None => break,
            Some((x, y)) => {
                let color = renderer.render(&scene, &scene.camera, x as f32, y as f32);
                frame_buffer.set_pixel(x, y, color);
                progress_logger.log(frame_buffer.progress());
            }
        }
    }
    progress_logger.log_end();

    let path = Path::new("output.bmp");
    frame_buffer.save_as_bmp(path).unwrap();
    println!("Saved to: {}", path.display());
}
