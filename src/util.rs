use crate::{octree::AABB, vector::Vec3};

pub struct Ray {
    pub origin: Vec3<f32>,
    pub dir: Vec3<f32>,
}

impl Ray {
    pub fn new(origin: Vec3<f32>, dir: Vec3<f32>) -> Self {
        Self { origin, dir }
    }
}

#[derive(Clone, Copy)]
pub struct Hit {
    pub dist: f32,
    pub origin: Vec3<f32>,
    pub point: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub color: Vec3<u8>,
}
impl Hit {
    pub fn new(
        dist: f32,
        origin: Vec3<f32>,
        point: Vec3<f32>,
        normal: Vec3<f32>,
        color: Vec3<u8>,
    ) -> Self {
        Self {
            dist,
            origin,
            point,
            normal,
            color,
        }
    }
    pub fn replace_if_closer(&mut self, other: Hit) {
        if other.dist < self.dist {
            self.dist = other.dist;
            self.origin = other.origin;
            self.point = other.point;
            self.normal = other.normal;
            self.color = other.color;
        }
    }
}

pub fn correct_normal(normal: Vec3<f32>, dir: &Vec3<f32>) -> Vec3<f32> {
    let inverse = normal * -1.0;
    return if normal.dot(&dir) < inverse.dot(&dir) {
        inverse
    } else {
        normal
    };
}

pub trait Shape {
    fn is_inside_aabb(&self, aabb: &AABB) -> bool;
    fn hit(&self, ray: &Ray) -> Option<Hit>;
    fn aabb(&self) -> AABB;
}
