use crate::{
    resolution::{AALevel, Resolution},
    scene_readers::{read_scene, Scene},
};
use std::{
    num::NonZeroUsize,
    path::{Path, PathBuf},
    time,
};

pub fn get_scene(path: &Path) -> Result<Scene, String> {
    let scene = read_scene(&path).unwrap();
    scene.print_stats();
    Ok(scene)
}

pub fn get_resolution() -> Resolution {
    let resolution = Resolution::new(
        NonZeroUsize::new(700).unwrap(),
        NonZeroUsize::new(700).unwrap(),
        AALevel::new(1).unwrap(),
    );
    resolution
}

pub fn get_window(resolution: &Resolution) -> minifb::Window {
    let mut window = minifb::Window::new(
        "Test - ESC to exit",
        resolution.width.get(),
        resolution.height.get(),
        minifb::WindowOptions::default(),
    )
    .unwrap();
    window
        .update_with_buffer(
            &vec![0; resolution.width.get() * resolution.height.get()],
            resolution.width.get(),
            resolution.height.get(),
        )
        .unwrap();
    window.limit_update_rate(Some(time::Duration::from_micros(16600)));
    window
}

pub struct Argv {
    pub input_file: PathBuf,
    pub output_file: Option<PathBuf>,
}

impl Argv {
    pub fn new() -> Result<Self, String> {
        let argv = std::env::args().collect::<Vec<_>>();
        if argv.len() <= 1 {
            return Err(format!(
                "Usage: {} <scene.[rt,obj,blend]>",
                argv.get(0).unwrap()
            ));
        }
        let input_file = argv.get(1).map(|s| PathBuf::from(s)).unwrap();
        let output_file = argv.get(2).map(|s| PathBuf::from(s));

        Ok(Self {
            input_file,
            output_file,
        })
    }
}
