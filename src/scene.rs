use crate::resolution::Resolution;
use crate::vector::Vec3;
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

pub struct Triangle {
    pub p0: Vec3<f32>,
    pub p1: Vec3<f32>,
    pub p2: Vec3<f32>,
    pub color: Vec3<u8>,
}

impl Triangle {
    // can this be done better?
    pub fn new(p0: Vec3<f32>, p1: Vec3<f32>, p2: Vec3<f32>, color: Vec3<u8>) -> Self {
        Self { p0, p1, p2, color }
    }

    pub fn hit(&self, ray: &Ray) -> Hit {
        // #ifndef USE_EIGEN
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;
        // let normal = edge1.cross(&edge2).normalized();
        // #endif

        let h = ray.dir.cross(&edge2);
        let a = edge1.dot(&h);
        if a > -f32::EPSILON && a < f32::EPSILON {
            return Hit::new(false); // Ray is parallel to this triangle.
        }
        let f = 1.0 / a;
        let s = ray.origin - self.p0;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return Hit::new(false);
        }
        let q = s.cross(&edge1);
        let v = f * ray.dir.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return Hit::new(false);
        }
        let t = f * edge2.dot(&q);
        if t < f32::EPSILON {
            // There is a line intersection but not a ray intersection
            return Hit::new(false);
        }

        // 	Hit hit(true);
        // hit.dist = (ray.dir * t).length();
        // hit.point = ray.origin * hit.dist;
        // hit.normal = correct_normal(normal, ray);
        // hit.color = color;
        return Hit::new(true);
    }
}

pub struct Camera {
    pub pos: Vec3<f32>,
    pub dir: Vec3<f32>,
    pub fov: f32,
    pub fow_tan: f32,
}

impl Camera {
    pub fn new(pos: Vec3<f32>, dir: Vec3<f32>, fov: f32) -> Self {
        let fow_tan = (fov * 0.5).tan();
        Self {
            pos,
            dir,
            fov,
            fow_tan,
        }
    }
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

pub struct Renderer {
    resolution: Resolution,
    aspect_ratio: f32,
}

impl Renderer {
    pub fn new(resolution: Resolution) -> Self {
        let aspect_ratio = resolution.width as f32 / resolution.height as f32;
        Self {
            resolution,
            aspect_ratio,
        }
    }
    pub fn ray_from_pixel(&self, camera: &Camera, x: f32, y: f32) -> Ray {
        let px =
            (2.0 * x / (self.resolution.width as f32) - 1.0) * self.aspect_ratio * camera.fow_tan;
        let py = (2.0 * y / (self.resolution.height as f32) - 1.0) * camera.fow_tan;

        let positive_x: Vec3<f32> = if camera.dir.x == 0.0 && camera.dir.z == 0.0 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            camera.dir.cross(&Vec3::new(0.0, 1.0, 0.0))
        };

        let negative_y = camera.dir.cross(&positive_x);

        let mut ray_dir = ((positive_x * px) + (negative_y * py)) + camera.dir;
        ray_dir.normalize();

        return Ray::new(camera.pos, ray_dir);
    }

    pub fn render(&self, scene: &Scene, camera: &Camera, x: f32, y: f32) -> Vec3<u8> {
        let mut hit = Hit::new(false);

        let ray = self.ray_from_pixel(camera, x, y);
        for triangle in &scene.triangles {
            let triangle_hit = triangle.hit(&ray);
            if triangle_hit.hit {
                hit = triangle_hit;
            }
        }
        if hit.hit {
            return Vec3::homogeneous(255);
        }
        return scene.background_color;
    }
    // pub fn render(self, scene: &Scene, camera: &Camera) -> Vec<u8> {
    //     let mut pixels: Vec<u8> = Vec::new();
    //     for y in 0..self.resolution.height {
    //         for x in 0..self.resolution.width {
    //             let ray = self.ray_from_pixel(camera, x as f32, y as f32);
    //             let mut hit = Hit::new(false);
    //             for triangle in &scene.triangles {
    //                 let triangle_hit = triangle.hit(ray);
    //                 if triangle_hit.hit {
    //                     hit = triangle_hit;
    //                 }
    //             }
    //             let color = match hit.hit {
    //                 true => Vec3::new(255, 0, 0),
    //                 false => scene.background_color,
    //             };
    //             pixels.push(color.x);
    //             pixels.push(color.y);
    //             pixels.push(color.z);
    //         }
    //     }
    //     return pixels;
    // }
}
