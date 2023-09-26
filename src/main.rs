extern crate bmp;
mod camera;
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
use std::num::NonZeroUsize;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use util::PositiveNonzeroF32;
use vector::Vec3;

fn get_rt_file() -> Option<PathBuf> {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        println!("Usage: {} <scene.rt>", argv.get(0).unwrap());
        return None;
    }
    Path::new(argv.get(1).unwrap()).canonicalize().ok()
}

fn span_thread(
    tx: Sender<(usize, usize, Vec3<u8>)>,
    renderer: Renderer,
    scene: Arc<Scene>,
    frame_buffer: Arc<Mutex<FrameBuffer>>,
) -> std::thread::JoinHandle<()> {
    return std::thread::spawn(move || loop {
        let mut frame_buffer = frame_buffer.lock().unwrap();
        let coordinate = frame_buffer.get_coordinate();
        drop(frame_buffer); // unlock mutex as soon as possible

        match coordinate {
            None => break,
            Some((x, y)) => {
                let color = renderer.render(&scene, &scene.camera, x as f32, y as f32);
                tx.send((x, y, color)).unwrap();
            }
        }
    });
}

fn main() {
    let scene_path = get_rt_file().unwrap();
    let scene = Scene::new(scene_path.as_path()).unwrap();
    let resolution = Resolution::new(500, 500).unwrap();
    let frame_buffer = FrameBuffer::new(resolution).unwrap();
    let renderer = Renderer::new(resolution);
    let mut progress_logger =
        ProgressLogger::new("Rendering", PositiveNonzeroF32::new(0.1).unwrap(), 1);

    let (tx, rx) = mpsc::channel();
    let scene = Arc::new(scene);
    let frame_buffer = Arc::new(Mutex::new(frame_buffer));
    let num_threads = std::thread::available_parallelism().unwrap_or(NonZeroUsize::new(8).unwrap());
    println!("Using {} threads", &num_threads);

    for _ in 0..num_threads.get() {
        let scene = scene.clone();
        let frame_buffer = frame_buffer.clone();
        let renderer = renderer.clone();
        let tx = tx.clone();
        let _ = span_thread(tx, renderer, scene, frame_buffer);
    }
    drop(tx);

    for (x, y, color) in rx {
        let mut frame_buffer = frame_buffer.lock().unwrap();
        frame_buffer.set_pixel(x, y, color);
        progress_logger.log(frame_buffer.progress());
    }
    progress_logger.log_end();

    let path = Path::new("output.bmp");
    let frame_buffer = frame_buffer.lock().unwrap();
    frame_buffer.save_as_bmp(path).unwrap();
    println!("Saved to: {}", path.display());
}
