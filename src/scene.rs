use crate::light::Light;
use crate::num::Float0to1;
use crate::octree::Octree;
use crate::vector::Vec3;
use crate::{camera::Camera, triangle::Triangle};
use std::io::{self, BufRead};

fn parse_rgb(s: &str) -> Option<Vec3<u8>> {
    let numbers: Vec<&str> = s.split(",").collect();
    if numbers.iter().count() != 3 {
        return None;
    }

    let r = numbers[0].parse::<u8>().ok()?;
    let g = numbers[1].parse::<u8>().ok()?;
    let b = numbers[2].parse::<u8>().ok()?;

    return Some(Vec3::new(r, g, b));
}

fn parse_vec3(s: &str, should_be_normalized: bool) -> Option<Vec3<f32>> {
    let parts: Vec<&str> = s.split(",").collect();
    if parts.len() != 3 {
        return None;
    }
    let x = parts[0].parse::<f32>().ok()?;
    let y = parts[1].parse::<f32>().ok()?;
    let z = parts[2].parse::<f32>().ok()?;

    return match should_be_normalized {
        true => Vec3::unit(x, y, z),
        false => Some(Vec3::new(x, y, z)),
    };
}

fn parse_triangle(t: Vec<&str>) -> Option<Triangle> {
    if t.len() != 5 {
        return None;
    }
    if t[0] != "tr" {
        return None;
    }
    let v0 = parse_vec3(t[1], false)?;
    let v1 = parse_vec3(t[2], false)?;
    let v2 = parse_vec3(t[3], false)?;
    let color = parse_rgb(t[4])?;

    return Some(Triangle::new(v0, v1, v2, color));
}

fn parse_light(blocks: Vec<&str>) -> Option<Light> {
    if blocks.get(0) != Some(&"l") {
        return None;
    }

    let origin = parse_vec3(blocks.get(1)?, false)?;
    let intensity = blocks.get(2)?.parse::<f32>().ok()?;
    let intensity = Float0to1::new(intensity)?;
    let color = parse_rgb(blocks.get(3)?)?;

    return Some(Light::new(origin, intensity, color));
}

fn parse_camera(blocks: Vec<&str>) -> Option<Camera> {
    if blocks.get(0) != Some(&"c") {
        return None;
    }
    let origin = parse_vec3(blocks.get(1)?, false)?;
    let direction = parse_vec3(blocks.get(2)?, true)?;
    let fov = blocks.get(3)?.parse::<f32>().ok()?;

    return Some(Camera::new(origin, direction, fov));
}

fn parse_ambient(blocks: Vec<&str>) -> Option<Light> {
    if blocks.get(0) != Some(&"A") {
        return None;
    }
    if blocks.len() != 3 {
        return None;
    }
    let intensity = blocks.get(1)?.parse::<f32>().ok()?;
    let color = parse_rgb(blocks.get(2)?)?;

    return Some(Light::new(
        Vec3::homogeneous(0.0),
        Float0to1::new(intensity)?,
        color,
    ));
}

pub struct Scene {
    pub camera: Camera, // TODO: should be plural
    pub triangles: Octree<Triangle>,
    pub lights: Vec<Light>,
    pub ambient: Light,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            triangles: Octree::new(vec![]),
            ambient: Light::new(
                Vec3::homogeneous(0.0),
                Float0to1::new(0.0).unwrap(),
                Vec3::homogeneous(0),
            ),
            lights: Vec::new(),
            camera: Camera::new(
                Vec3::new(35.0, 18.0, 31.0),
                Vec3::new(-0.7247, -0.18, -0.78087),
                70.0,
            ),
        }
    }
}

impl Scene {
    pub fn new(path: &std::path::Path) -> Result<Self, String> {
        let file = std::fs::File::open(path).or(Err("Could not open file"))?;
        let lines = io::BufReader::new(file).lines();
        let mut triangles: Vec<Triangle> = Vec::new();
        let mut self_ = Self::default();

        for line in lines {
            let line = line.map_err(|_| "Could not read line: ".to_string())?;
            self_
                .parse_line(&line, &mut triangles)
                .map_err(|_| "Could not parse line: ".to_string() + &line)?;
        }
        self_.triangles = Octree::new(triangles);
        return Ok(self_);
    }

    pub fn void(&self) -> Vec3<u8> {
        Vec3::homogeneous(0)
    }

    fn parse_line(&mut self, line: &str, triangles: &mut Vec<Triangle>) -> Result<(), ()> {
        if line.len() == 0 {
            return Ok(());
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 0 {
            return Ok(());
        }
        return match parts[0] {
            "tr" => {
                let triangle = parse_triangle(parts).ok_or(())?;
                triangles.push(triangle);
                Ok(())
            }
            "l" => {
                let light = parse_light(parts).ok_or(())?;
                self.lights.push(light);
                Ok(())
            }
            "c" => {
                self.camera = parse_camera(parts).ok_or(())?;
                Ok(())
            }
            "A" => {
                self.ambient = parse_ambient(parts).ok_or(())?;
                Ok(())
            }
            _ => return Ok(()),
        };
    }
}
