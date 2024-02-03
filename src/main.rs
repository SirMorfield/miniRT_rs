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
mod threading;
mod triangle;
mod util;
mod vector;

use crate::threading::MultiThreadedRenderer;
use num::PowerOf2;
use resolution::Resolution;
use std::num::NonZeroUsize;
use std::path::Path;
use std::path::PathBuf;

fn get_render_file() -> Option<PathBuf> {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        println!("Usage: {} <scene.[rt,obj,blend]>", argv.get(0).unwrap());
        return None;
    }
    Path::new(argv.get(1).unwrap()).canonicalize().ok()
}
//https://crates.io/crates/minifb
fn main() {
    let scene_path = get_render_file().unwrap();
    let resolution = Resolution::new(
        NonZeroUsize::new(700).unwrap(),
        NonZeroUsize::new(700).unwrap(),
        PowerOf2::new(4).unwrap(),
    );
    resolution.print();
    let mut renderer = MultiThreadedRenderer::new(resolution, scene_path);
    renderer.render();
    let path = Path::new("output.bmp");
    renderer
        .frame_buffer
        .lock()
        .unwrap()
        .save_as_bmp(path)
        .unwrap();
    println!("Saved to: ./{}", path.display());
}
