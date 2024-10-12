use std::num::NonZeroUsize;

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

impl PartialEq for PixelReq {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PixelReq {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub const PIXEL_BUFFER_SIZE: usize = 200;
pub type PixelReqBuffer = [Option<PixelReq>; PIXEL_BUFFER_SIZE];
pub type PixelResBuffer = [Option<PixelRes>; PIXEL_BUFFER_SIZE];

pub trait ExitOnError<T> {
    fn exit_with(self, message: &str) -> T;
}

impl<T, E: std::fmt::Display> ExitOnError<T> for Result<T, E> {
    fn exit_with(self, message: &str) -> T {
        match self {
            Ok(val) => val,
            Err(_) => {
                eprintln!("{}", message);
                std::process::exit(1);
            }
        }
    }
}

pub fn threads() -> usize {
    return std::thread::available_parallelism()
        .unwrap_or(NonZeroUsize::new(8).unwrap())
        .get();
}

pub fn split(buf: &PixelReqBuffer, n: usize) -> Vec<PixelReqBuffer> {
    let mut result = Vec::with_capacity(std::cmp::min(n, PIXEL_BUFFER_SIZE));
    let chunk_size = PIXEL_BUFFER_SIZE / n;

    for i in 0..n {
        let start = i * chunk_size;
        let end = if i == n - 1 { buf.len() } else { (i + 1) * chunk_size };
        let mut part_buf = [None; PIXEL_BUFFER_SIZE];
        for j in start..end {
            part_buf[j - start] = buf[j];
        }
        result.push(part_buf);
    }
    return result;
}

mod split_test {
    use super::*;

    #[test]
    fn test_split() {
        const x: usize = PIXEL_BUFFER_SIZE;
        let mut buf = [None; x];

        for i in 0..x {
            buf[i] = Some(PixelReq::new(i, i));
        }

        for split_size in vec![1, 2, 3, 4, x - 2, x - 1, x, x + 1, x + 2] {
            let parts = split(&buf, split_size);

            for req in buf.iter() {
                if req.is_none() {
                    continue;
                }
                let mut found = false;
                for part in parts.iter() {
                    for part_req in part.iter() {
                        if req == part_req {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        break;
                    }
                }
                assert!(found);
            }
        }
    }
}
