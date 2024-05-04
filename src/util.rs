use serde::{Deserialize, Serialize};

use crate::{octree::AABB, vector::Point};

pub struct Ray {
    pub origin: Point<f32>,
    pub dir: Point<f32>,
}

impl Ray {
    pub fn new(origin: Point<f32>, dir: Point<f32>) -> Self {
        Self { origin, dir }
    }
}

#[derive(Clone, Copy)]
pub struct Hit {
    pub dist: f32,
    pub origin: Point<f32>,
    pub point: Point<f32>,
    pub normal: Point<f32>,
    pub color: Point<u8>,
}
impl Hit {
    pub fn new(dist: f32, origin: Point<f32>, point: Point<f32>, normal: Point<f32>, color: Point<u8>) -> Self {
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

pub fn correct_normal(normal: Point<f32>, dir: &Point<f32>) -> Point<f32> {
    let inverse = normal * -1.0;
    return if normal.dot(&dir) < inverse.dot(&dir) {
        inverse
    } else {
        normal
    };
}

pub trait Intersect {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

pub trait Shape {
    fn is_inside_aabb(&self, aabb: &AABB) -> bool;
    fn aabb(&self) -> AABB;
}

#[allow(dead_code)]
pub fn fps_to_duration(fps: u32) -> std::time::Duration {
    std::time::Duration::from_micros(1_000_000 / fps as u64)
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct PixelRes {
    pub x: usize,
    pub y: usize,
    pub color: Point<u8>,
}

impl PixelRes {
    pub fn new(x: usize, y: usize, color: Point<u8>) -> Self {
        Self { x, y, color }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct PixelReq {
    pub x: usize,
    pub y: usize,
}

impl PixelReq {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub const PIXEL_BUFFER_SIZE: usize = 200;
pub type PixelReqBuffer = [Option<PixelReq>; PIXEL_BUFFER_SIZE];
pub type PixelResBuffer = [Option<PixelRes>; PIXEL_BUFFER_SIZE];
