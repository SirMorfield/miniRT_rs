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
use threadpool::ThreadPool;

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

    pub fn render(&mut self, scene: &Arc<RwLock<Scene>>, log: bool) {
        self.reset_progress();
        let (tx, rx) = mpsc::channel();
        let mut fb = self.frame_buffer.lock().unwrap();
        let pool = ThreadPool::new(self.num_threads);

        loop {
            let pixel = fb.get_coordinate();
            if pixel == None {
                break;
            }
            let pixel = pixel.unwrap();
            let tx = tx.clone();
            let renderer = self.renderer.clone();
            let scene = scene.clone();
            pool.execute(move || {
                let (x, y) = pixel;
                let scene = scene.read().unwrap();
                let color = renderer.render(&scene, &scene.camera, x as f32, y as f32);
                tx.send((x, y, color)).unwrap();
            });
        }
        drop(tx);

        for (x, y, color) in rx {
            fb.set_pixel(x, y, color);
            if log {
                self.progress_logger.log(fb.progress());
            }
        }
        if log {
            self.progress_logger.log_end();
        }
        let scene = scene.read().unwrap();
        if scene.file_type == FileType::Obj {
            fb.flip(Flip::Horizontal);
        }
    }
}
