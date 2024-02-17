use std::{
    num::NonZeroUsize,
    path::{Path, PathBuf},
};

use crate::{
    resolution::{AALevel, Resolution},
    scene_readers::{read_scene, Scene},
};

pub fn get_scene() -> Result<Scene, String> {
    let path = get_render_file().unwrap();
    read_scene(&path)
}

pub fn get_resolution() -> Resolution {
    let resolution = Resolution::new(
        NonZeroUsize::new(700).unwrap(),
        NonZeroUsize::new(700).unwrap(),
        AALevel::new(1).unwrap(),
    );
    resolution
}

fn get_render_file() -> Option<PathBuf> {
    let argv = std::env::args().collect::<Vec<_>>();
    if argv.len() != 2 {
        println!("Usage: {} <scene.[rt,obj,blend]>", argv.get(0).unwrap());
        return None;
    }
    Path::new(argv.get(1).unwrap()).canonicalize().ok()
}
