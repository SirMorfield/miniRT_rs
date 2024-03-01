use crate::frame_buffer;
use crate::num;
use crate::progress_logger;
use crate::renderer;
use crate::resolution;
use crate::scene_readers::FileType;
use crate::scene_readers::Scene;
use crate::vector::Point;
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

    pub fn render(&mut self, scene: &Arc<RwLock<Scene>>, log: bool) {
        self.reset_progress();
        let (tx, rx) = mpsc::channel();
        for _ in 0..self.num_threads {
            let fb = self.frame_buffer.clone();
            let tx = tx.clone();
            let renderer = self.renderer.clone();
            let scene = scene.clone();
            std::thread::spawn(move || {
                let scene = scene.read().unwrap();
                loop {
                    const MAX_COORDINATES: usize = 1000;
                    let mut fb = fb.lock().unwrap();
                    let coordinates = fb.get_coordinates::<MAX_COORDINATES>();
                    drop(fb); // unlock mutex as soon as possible

                    let mut colors = [None; MAX_COORDINATES];
                    let mut end = false;

                    // Egypt is never far
                    for (i, coordinate) in coordinates.iter().enumerate() {
                        match coordinate {
                            None => end = true,
                            Some((x, y)) => {
                                let color =
                                    renderer.render(&scene, &scene.camera, *x as f32, *y as f32);
                                colors[i] = Some((*x, *y, color));
                            }
                        }
                    }
                    tx.send(colors).unwrap();
                    if end {
                        return;
                    }
                }
            });
        }
        drop(tx);

        for colors in rx {
            for (x, y, color) in colors.iter().filter_map(|c| *c) {
                let mut fb = self.frame_buffer.lock().unwrap();
                fb.set_pixel(x, y, color);
                let progress = fb.progress();
                drop(fb);
                if log {
                    self.progress_logger.log(progress);
                }
            }
        }
        self.progress_logger.log_end();
        let scene = scene.read().unwrap();
        if scene.file_type == FileType::Obj {
            self.frame_buffer.lock().unwrap().flip(Flip::Horizontal);
        }
    }
}
