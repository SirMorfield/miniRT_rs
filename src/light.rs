use crate::num::Float0to1;
use crate::vector::Point;

pub struct Light {
    pub origin: Point<f32>,
    intensity: f32,
    pub color: Point<u8>,
}

impl Light {
    pub fn new(origin: Point<f32>, intensity: Float0to1, color: Point<u8>) -> Self {
        Self {
            origin,
            intensity: intensity.get(),
            color,
        }
    }
    pub fn absolute_color(&self) -> Point<u8> {
        return Point::new(
            (self.color.x as f32 * self.intensity) as u8,
            (self.color.y as f32 * self.intensity) as u8,
            (self.color.z as f32 * self.intensity) as u8,
        );
    }
    pub fn relative_intensity(&self, point: &Point<f32>, normal: &Point<f32>) -> f32 {
        let to_light = (self.origin - *point).to_normalized();

        let intensity = self.intensity * normal.dot(&to_light).max(0.0);
        return intensity.min(1.0);
    }
}
