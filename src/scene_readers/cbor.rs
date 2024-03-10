use super::Scene;
use std::{fs::File, path::Path};

pub fn read_cbor(path: &Path) -> Result<Scene, String> {
    let now = std::time::Instant::now();
    let scene_file = File::open(path).map_err(|e| e.to_string())?;
    let mut scene: Scene = serde_cbor::from_reader(scene_file).map_err(|e| e.to_string())?;
    scene.load_duration = now.elapsed();
    return Ok(scene);
}
