use crate::frame_buffer;
use crate::num;
use crate::progress_logger;
use crate::renderer;
use crate::resolution;
use crate::scene_readers::get_scene;
use frame_buffer::Flip;
use frame_buffer::FrameBuffer;
use num::PositiveNonzeroF32;
use progress_logger::ProgressLogger;
use renderer::Renderer;
use resolution::Resolution;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};

pub struct MultiThreadedRenderer {
    resolution: Resolution,
    scene_path: PathBuf,
    pub frame_buffer: Arc<Mutex<FrameBuffer>>,
}

impl MultiThreadedRenderer {
    pub fn new(resolution: Resolution, scene_path: PathBuf) -> Self {
        Self {
            resolution,
            scene_path,
            frame_buffer: Arc::new(Mutex::new(FrameBuffer::new(resolution).unwrap())),
        }
    }
    pub fn render(&mut self) {
        let mut progress_logger =
            ProgressLogger::new("Rendering", PositiveNonzeroF32::new(0.1).unwrap(), 1);
        let num_threads = std::thread::available_parallelism()
            .unwrap_or(NonZeroUsize::new(8).unwrap())
            .get();
        println!("Using {num_threads} threads");

        let scene = Arc::new(get_scene(&self.scene_path).unwrap());
        scene.print_stats();
        let renderer = Arc::new(Renderer::new(self.resolution));

        // start
        let (tx, rx) = mpsc::channel();
        for _ in 0..num_threads {
            let fb = self.frame_buffer.clone();
            let tx = tx.clone();
            let renderer = renderer.clone();
            let scene = scene.clone();
            std::thread::spawn(move || loop {
                let mut fb = fb.lock().unwrap();
                let coordinate = fb.get_coordinate();
                drop(fb); // unlock mutex as soon as possible

                match coordinate {
                    None => break,
                    Some((x, y)) => {
                        let color = renderer.render(&scene, &scene.camera, x as f32, y as f32);
                        tx.send((x, y, color)).unwrap();
                    }
                }
            });
        }
        drop(tx);

        for (x, y, color) in rx {
            let mut fb = self.frame_buffer.lock().unwrap();
            fb.set_pixel(x, y, color);
            let progress = fb.progress();
            drop(fb);
            progress_logger.log(progress);
        }
        progress_logger.log_end();

        if self.scene_path.extension().unwrap() == "obj" {
            self.frame_buffer.lock().unwrap().flip(Flip::Horizontal);
        }
        // self.frame_buffer = *frame_buffer;
    }
}
