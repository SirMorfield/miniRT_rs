use crate::num::PositiveNonzeroF32;
use crate::vector::Vec3;

pub struct Light {
    pub origin: Vec3<f32>,
    intensity: f32,
    pub color: Vec3<u8>,
}

impl Light {
    pub fn new(origin: Vec3<f32>, intensity: PositiveNonzeroF32, color: Vec3<u8>) -> Self {
        Self {
            origin,
            intensity: intensity.get(),
            color,
        }
    }
    pub fn relative_intensity(&self, point: &Vec3<f32>, normal: &Vec3<f32>) -> f32 {
        let to_light = (self.origin - *point).to_normalized();

        let intensity = self.intensity * normal.dot(&to_light).max(0.0);
        return intensity.min(1.0);
    }
    pub fn as_float(&self) -> Vec3<f32> {
        return Vec3::new(
            self.color.x as f32,
            self.color.y as f32,
            self.color.z as f32,
        );
    }
}
