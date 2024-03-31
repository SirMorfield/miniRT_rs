use super::Scene;
use std::{fs::File, path::Path, time::Instant, io::BufReader};
use crate::scene_readers::FileType::Cbor;

pub fn read_cbor(path: &Path) -> Result<Scene, String> {
    let start = Instant::now();
    let scene_file = File::open(path).map_err(|e| e.to_string())?;
    let mut scene_file_buf = BufReader::new(scene_file);
    let mut scene: Scene = serde_cbor::from_reader(&mut scene_file_buf).map_err(|e| e.to_string())?;
    scene.file_type = Cbor;
    scene.load_duration = start.elapsed();
    return Ok(scene);
}
