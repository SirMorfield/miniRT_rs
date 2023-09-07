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

pub struct Scene {
    pub camera: Camera, // should be plural
    pub triangles: Vec<Triangle>,
    pub background_color: Vec3<u8>,
}

impl Scene {
    pub fn default() -> Self {
        Self {
            triangles: Vec::new(),
            background_color: Vec3::new(0, 0, 0),

            // c   35.0,18.0,31.0         -0.7247,0.0,-0.78087         70
            camera: Camera::new(
                Vec3::new(35.0, 18.0, 31.0),
                Vec3::new(-0.7247, 0.0, -0.78087),
                70.0,
            ),
        }
    }
    pub fn new(path: &std::path::Path) -> Result<Self, &str> {
        let file = std::fs::File::open(path).or(Err("Could not open file"))?;
        let lines = io::BufReader::new(file).lines();

        let mut self_ = Self::default();
        // TODO: Egypt is never far
        for line in lines {
            match line {
                Ok(line) => {
                    match self_.parse_line(&line) {
                        Ok(_) => (),
                        Err(_) => return Err("Could not parse line"),
                    };
                }
                Err(_) => return Err("Could not read line"),
            }
        }
        return Ok(self_);
    }
    fn parse_line(&mut self, line: &str) -> Result<(), ()> {
        if line.len() == 0 {
            return Ok(());
        }
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() == 0 {
            return Ok(());
        }
        return match parts[0] {
            "tr" => {
                let triangle = parse_triangle(parts).ok_or(())?;
                self.triangles.push(triangle);
                Ok(())
            }
            _ => return Ok(()),
        };
    }
}
