use crate::vector::Vec3;

pub struct Ray {
    pub origin: Vec3<f32>,
    pub dir: Vec3<f32>,
}

impl Ray {
    pub fn new(origin: Vec3<f32>, dir: Vec3<f32>) -> Self {
        Self { origin, dir }
    }
}

pub struct Hit {
    pub hit: bool,
}

impl Hit {
    pub fn new(hit: bool) -> Self {
        Self { hit }
    }
}
