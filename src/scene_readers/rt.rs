use super::{FileType, Scene};
use crate::light::Light;
use crate::num::Float0to1;
use crate::octree::Octree;
use crate::vector::Point;
use crate::{camera::Camera, triangle::Triangle};
use std::io::{self, BufRead};

fn parse_rgb(s: &str) -> Option<Point<u8>> {
    let numbers: Vec<&str> = s.split(",").collect();
    if numbers.iter().count() != 3 {
        return None;
    }

    let r = numbers[0].parse::<u8>().ok()?;
    let g = numbers[1].parse::<u8>().ok()?;
    let b = numbers[2].parse::<u8>().ok()?;

    return Some(Point::new(r, g, b));
}

fn parse_point(s: &str, should_be_normalized: bool) -> Option<Point<f32>> {
    let parts: Vec<&str> = s.split(",").collect();
    if parts.len() != 3 {
        return None;
    }
    let x = parts[0].parse::<f32>().ok()?;
    let y = parts[1].parse::<f32>().ok()?;
    let z = parts[2].parse::<f32>().ok()?;

    return match should_be_normalized {
        true => Point::unit(x, y, z),
        false => Some(Point::new(x, y, z)),
    };
}

fn parse_triangle(t: Vec<&str>) -> Option<Triangle> {
    if t.len() != 5 {
        return None;
    }
    if t[0] != "tr" {
        return None;
    }
    let v0 = parse_point(t[1], false)?;
    let v1 = parse_point(t[2], false)?;
    let v2 = parse_point(t[3], false)?;
    let color = parse_rgb(t[4])?;

    return Some(Triangle::new(v0, v1, v2, color));
}

fn parse_light(blocks: Vec<&str>) -> Option<Light> {
    if blocks.get(0) != Some(&"l") {
        return None;
    }

    let origin = parse_point(blocks.get(1)?, false)?;
    let intensity = blocks.get(2)?.parse::<f32>().ok()?;
    let intensity = Float0to1::new(intensity)?;
    let color = parse_rgb(blocks.get(3)?)?;

    return Some(Light::new(origin, intensity, color));
}

fn parse_camera(blocks: Vec<&str>) -> Option<Camera> {
    if blocks.get(0) != Some(&"c") {
        return None;
    }
    let origin = parse_point(blocks.get(1)?, false)?;
    let direction = parse_point(blocks.get(2)?, true)?;
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

    return Some(Light::new(Point::homogeneous(0.0), Float0to1::new(intensity)?, color));
}

pub fn read_rt(path: &std::path::Path) -> Result<super::Scene, String> {
    let now = std::time::Instant::now();
    if !path.display().to_string().ends_with(".rt") {
        return Err("File must end with .rt".into());
    }
    let file = std::fs::File::open(path).or(Err("Could not open file"))?;
    let lines = io::BufReader::new(file).lines();
    let mut triangles: Vec<Triangle> = Vec::new();

    let mut ambient = Light::new(
        Point::homogeneous(0.0),
        Float0to1::new(0.0).unwrap(),
        Point::homogeneous(0),
    );
    let mut lights: Vec<Light> = Vec::new();
    let mut camera = Camera::new(Point::new(35.0, 18.0, 31.0), Point::new(-0.7247, -0.18, -0.78087), 70.0);

    for line in lines {
        let line = line.map_err(|_| "Could not read line".to_string())?;

        if line.len() == 0 {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 0 {
            continue;
        }
        match parts[0] {
            "tr" => {
                if let Some(t) = parse_triangle(parts) {
                    triangles.push(t);
                }
            }
            "l" => {
                if let Some(l) = parse_light(parts) {
                    lights.push(l);
                }
            }
            "c" => {
                if let Some(c) = parse_camera(parts) {
                    camera = c;
                }
            }
            "A" => {
                if let Some(a) = parse_ambient(parts) {
                    ambient = a;
                }
            }
            _ => (),
        }
    }
    let triangles = Octree::new(triangles);
    let parse_duration = now.elapsed();
    return Ok(Scene::new(
        camera,
        triangles,
        lights,
        ambient,
        parse_duration,
        FileType::Rt,
    ));
}
