use crate::{
    resolution::{AALevel, Resolution},
    scene_readers::{read_scene, Scene},
};
use std::{
    num::NonZeroUsize,
    path::{Path, PathBuf},
};
use std::str::FromStr;
use strum_macros::{EnumString, EnumIter, Display};
use strum::IntoEnumIterator;


pub fn get_scene(path: &Path) -> Result<Scene, String> {
    let scene = read_scene(&path).unwrap();
    scene.print_stats();
    Ok(scene)
}

pub fn get_resolution() -> Resolution {
    let resolution = Resolution::new(
        NonZeroUsize::new(70).unwrap(),
        NonZeroUsize::new(70).unwrap(),
        AALevel::new(1).unwrap(),
    );
    resolution.print();
    resolution
}

#[derive(Debug,PartialEq,Display,EnumString,EnumIter)]
pub enum Mode {
    ToFile,
    Window,
    NetClient,
    NetServer,
}

#[derive(Debug)]
pub struct Argv {
    pub mode: Mode,
    pub input_file: PathBuf,
    pub output_file: Option<PathBuf>,
    pub address: Option<String>,
}

impl Argv {
    pub fn new() -> Self {
        let argv = std::env::args().collect::<Vec<_>>();
        if argv.len() <= 1 {
            error(&argv);
        }
        let mode = Mode::from_str(argv.get(1).unwrap());
        if mode.is_err() {
            error(&argv);
        }
        let mode = mode.unwrap();

        if mode == Mode::NetClient || mode == Mode::NetServer {
            if argv.len() < 5 {
                error(&argv);
            }
            let input_file = argv.get(2).map(|s| PathBuf::from(s)).unwrap();
            let output_file = argv.get(3).map(|s| PathBuf::from(s));
            let address = argv.get(4).map(|s| s.to_string());
            return Self {
                mode,
                input_file,
                address,
                output_file,
            };
        }

        let input_file = argv.get(2).map(|s| PathBuf::from(s)).unwrap();
        let output_file = argv.get(3).map(|s| PathBuf::from(s));

        Self {
            mode,
            input_file,
            output_file,
            address: None,
        }
    }
}

fn error(argv: &Vec<String>) -> ! {
    let modes = Mode::iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", ");
    println!("Usage: {} <{modes}> <scene.[rt,obj,blend]> <output_file.[bmp,cbor]> [<address>]", argv.get(0).unwrap());
    std::process::exit(1);
}