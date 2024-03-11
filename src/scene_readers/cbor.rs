use super::Scene;
use std::{fs::File, path::Path};

pub fn read_cbor(path: &Path) -> Result<Scene, String> {
    let now = std::time::Instant::now();
    let scene_file = File::open(path).map_err(|e| e.to_string())?;
    let mut scene_file_buf = std::io::BufReader::new(scene_file);
    let mut scene: Scene = serde_cbor::from_reader(&mut scene_file_buf).map_err(|e| e.to_string())?;
    scene.load_duration = now.elapsed();
    return Ok(scene);
}
