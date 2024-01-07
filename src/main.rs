extern crate bmp;
extern crate num_integer;

mod camera;
mod frame_buffer;
mod helpers;
mod light;
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

use frame_buffer::Flip;
use frame_buffer::FrameBuffer;
use num::PositiveNonzeroF32;
use num::PowerOf2;
use progress_logger::ProgressLogger;
use renderer::Renderer;
use resolution::Resolution;
use scene_readers::{get_scene, Scene};
use std::num::NonZeroUsize;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use vector::Vec3;

fn get_render_file() -> Option<PathBuf> {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        println!("Usage: {} <scene.[rt,obj,blend]>", argv.get(0).unwrap());
        return None;
    }
    Path::new(argv.get(1).unwrap()).canonicalize().ok()
}

fn spawn_thread(
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

fn render_scene(
    scene: Arc<Scene>,
    frame_buffer: Arc<Mutex<FrameBuffer>>,
    resolution: Resolution,
) -> Receiver<(usize, usize, Vec3<u8>)> {
    let (tx, rx) = mpsc::channel();
    let renderer = Renderer::new(resolution);
    let num_threads = std::thread::available_parallelism()
        .unwrap_or(NonZeroUsize::new(8).unwrap())
        .get();
    println!("Using {num_threads} threads");

    for _ in 0..num_threads {
        let _ = spawn_thread(
            tx.clone(),
            renderer.clone(),
            scene.clone(),
            frame_buffer.clone(),
        );
    }
    drop(tx);
    return rx;
}

fn main() {
    let scene_path = get_render_file().unwrap();
    let scene = get_scene(&scene_path).unwrap();
    scene.print_stats();
    let resolution = Resolution::new(
        NonZeroUsize::new(500).unwrap(),
        NonZeroUsize::new(500).unwrap(),
        PowerOf2::new(4).unwrap(),
    );
    resolution.print();
    // scene.triangles.print(true, true, 10);
    let frame_buffer = Arc::new(Mutex::new(FrameBuffer::new(resolution).unwrap()));
    let mut progress_logger =
        ProgressLogger::new("Rendering", PositiveNonzeroF32::new(0.1).unwrap(), 1);

    let rx = render_scene(Arc::new(scene), frame_buffer.clone(), resolution);
    for (x, y, color) in rx {
        let mut frame_buffer = frame_buffer.lock().unwrap();
        frame_buffer.set_pixel(x, y, color);
        progress_logger.log(frame_buffer.progress());
    }
    progress_logger.log_end();

    let path = Path::new("output.bmp");
    let mut frame_buffer = frame_buffer.lock().unwrap();
    if scene_path.extension().unwrap() == "obj" {
        frame_buffer.flip(Flip::Horizontal);
    }
    frame_buffer.save_as_bmp(path).unwrap();
    println!("Saved to: ./{}", path.display());
}
