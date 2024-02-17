use crate::frame_buffer;
use crate::num;
use crate::progress_logger;
use crate::renderer;
use crate::resolution;
use crate::scene_readers::FileType;
use crate::scene_readers::Scene;
use frame_buffer::Flip;
use frame_buffer::FrameBuffer;
use num::PositiveNonzeroF32;
use progress_logger::ProgressLogger;
use renderer::Renderer;
use resolution::Resolution;
use std::num::NonZeroUsize;
use std::sync::RwLock;
use std::sync::{mpsc, Arc, Mutex};

pub struct MultiThreadedRenderer {
    progress_logger: ProgressLogger,
    num_threads: usize,
    renderer: Arc<Renderer>,
    // pub scene: Scene,
    pub frame_buffer: Arc<Mutex<FrameBuffer>>,
}

impl MultiThreadedRenderer {
    pub fn new(resolution: Resolution) -> Self {
        Self {
            num_threads: std::thread::available_parallelism()
                .unwrap_or(NonZeroUsize::new(8).unwrap())
                .get(),
            progress_logger: ProgressLogger::new(
                "Rendering",
                PositiveNonzeroF32::new(0.1).unwrap(),
                1,
            ),
            renderer: Arc::new(Renderer::new(resolution)),
            frame_buffer: Arc::new(Mutex::new(FrameBuffer::new(resolution).unwrap())),
        }
    }

    fn reset_progress(&mut self) {
        self.frame_buffer.lock().unwrap().reset_progress();
    }

    pub fn render(&mut self, scene: &Arc<RwLock<Scene>>) {
        self.reset_progress();
        let (tx, rx) = mpsc::channel();
        for _ in 0..self.num_threads {
            let fb = self.frame_buffer.clone();
            let tx = tx.clone();
            let renderer = self.renderer.clone();
            let scene = scene.clone();
            std::thread::spawn(move || loop {
                let mut fb = fb.lock().unwrap();
                let coordinate = fb.get_coordinate();
                drop(fb); // unlock mutex as soon as possible

                match coordinate {
                    None => break,
                    Some((x, y)) => {
                        let scene = scene.read().unwrap();
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
            self.progress_logger.log(progress);
        }
        self.progress_logger.log_end();

        let scene = scene.read().unwrap();
        if scene.file_type == FileType::Obj {
            self.frame_buffer.lock().unwrap().flip(Flip::Horizontal);
        }
    }
}
