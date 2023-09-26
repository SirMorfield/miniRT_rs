use crate::vector::Vec3;

pub struct Camera {
    pub pos: Vec3<f32>,
    pub dir: Vec3<f32>,
    pub fov: f32,
    pub fow_tan: f32,
}

impl Camera {
    pub fn new(pos: Vec3<f32>, dir: Vec3<f32>, fov: f32) -> Self {
        let fow_tan = (fov * 0.5).tan();
        Self {
            pos,
            dir: dir.to_normalized(),
            fov,
            fow_tan,
        }
    }
}
