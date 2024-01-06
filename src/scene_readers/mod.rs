use crate::num::Float0to1;
use crate::octree::Octree;
use crate::triangle::Triangle;
use crate::{camera::Camera, light::Light, vector::Vec3};

mod obj;
mod obj_trimesh;
mod rt;

pub struct Scene {
    pub camera: Camera, // TODO: should be plural
    pub triangles: Octree<Triangle>,
    pub lights: Vec<Light>,
    pub ambient: Light,
}

impl Scene {
    pub fn new(
        camera: Camera,
        triangles: Octree<Triangle>,
        lights: Vec<Light>,
        ambient: Light,
    ) -> Self {
        Self {
            camera,
            triangles,
            lights,
            ambient,
        }
    }

    pub fn void(&self) -> Vec3<u8> {
        Vec3::homogeneous(0)
    }

    pub fn print_stats(&self) {
        println!("Scene");
        println!("  Triangles: {}", self.triangles.shapes_count());
        println!("  Lights   : {}", self.lights.len());
    }
}

pub fn get_scene(path: &std::path::Path) -> Result<Scene, String> {
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
    let mut origin = avg + size / 2.0;
    origin.y = avg.y; // set camera to be at the same height as the model
    let dir = (avg - origin).to_normalized();

    // println!("min   : {:?}", min);
    // println!("max   : {:?}", max);
    // println!("size  : {:?}", size);
    // println!("avg   : {:?}", avg);
    // println!("origin: {:?}", origin);
    return Camera::new(origin, dir, 80.0);
}

pub fn default_ambient() -> Light {
    Light::new(
        Vec3::homogeneous(0.0),
        Float0to1::new(0.5).unwrap(),
        Vec3::homogeneous(255),
    )
}
