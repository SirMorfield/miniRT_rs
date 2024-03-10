use crate::num::Float0to1;
use crate::octree::Octree;
use crate::triangle::Triangle;
use crate::{camera::Camera, light::Light, vector::Point};
// use serde::{Deserialize, Serialize};
use std::time::Duration;

mod obj;
mod rt;

#[derive(PartialEq)]
pub enum FileType {
    Rt,
    Obj,
}

// #[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera, // TODO: should be plural
    pub triangles: Octree<Triangle>,
    pub lights: Vec<Light>,
    pub ambient: Light,
    pub load_duration: Duration,
    pub file_type: FileType,
}

impl Scene {
    pub fn new(
        camera: Camera,
        triangles: Octree<Triangle>,
        lights: Vec<Light>,
        ambient: Light,
        parse_duration: Duration,
        file_type: FileType,
    ) -> Self {
        Self {
            camera,
            triangles,
            lights,
            ambient,
            load_duration: parse_duration,
            file_type,
        }
    }

    pub fn void(&self) -> Point<u8> {
        Point::homogeneous(0)
    }

    #[allow(dead_code)]
    pub fn print_stats(&self) {
        println!("Scene");
        println!("  Loaded in: {:?}", self.load_duration);
        println!("  Triangles: {}", self.triangles.shapes_count());
        println!("  Lights   : {}", self.lights.len());
    }
}

pub fn read_scene(path: &std::path::Path) -> Result<Scene, String> {
    let display = path.display().to_string();

    if display.ends_with(".rt") {
        return rt::read_rt(&path);
    }
    if display.ends_with(".obj") {
        return obj::read_obj(&path);
    }
    return Err("Could not read file: ".to_string() + &display);
}

pub fn look_at(triangles: &Vec<Triangle>) -> Camera {
    let avg = triangles
        .iter()
        .map(|t| (t.p0 + t.p1 + t.p2) / 3.0)
        .reduce(|a, b| a + b)
        .unwrap()
        / triangles.len() as f32;
    let min = triangles
        .iter()
        .map(|t| t.p0.min_unsafe(t.p1).min_unsafe(t.p2))
        .reduce(|a, b| a.min_unsafe(b))
        .unwrap();
    let max = triangles
        .iter()
        .map(|t| t.p0.max_unsafe(t.p1).max_unsafe(t.p2))
        .reduce(|a, b| a.max_unsafe(b))
        .unwrap();

    let size = (max - min).length();
    let mut origin = avg + size / 5.0;
    origin.z = -(avg.z + size / 2.0).abs();
    let dir = (avg - origin).to_normalized();

    // println!("min   : {:?}", min);
    // println!("max   : {:?}", max);
    // println!("size  : {:?}", size);
    // println!("avg   : {:?}", avg);
    // println!("origin: {:?}", origin);

    return Camera::new(origin, dir, 80.0, size / 5.0, 0.1);
}

pub fn default_ambient() -> Light {
    Light::new(
        Point::homogeneous(0.0),
        Float0to1::new(0.1).unwrap(),
        Point::homogeneous(255),
    )
}
