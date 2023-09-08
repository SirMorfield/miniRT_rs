use crate::camera::Camera;
use crate::resolution::Resolution;
use crate::scene::Scene;
use crate::triangle::Triangle;
use crate::util::{Hit, Ray};
use crate::vector::Vec3;
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
        let mut hit: Option<Hit> = None;

        let ray = self.ray_from_pixel(camera, x, y);
        for triangle in &scene.triangles {
            let triangle_hit = triangle.hit(&ray);
            if !triangle_hit {
                continue;
            }
            let info = triangle.hit_info(&ray);

            match hit {
                None => hit = Some(info),
                Some(hit_info) => {
                    if info.dist < hit_info.dist {
                        hit = Some(info);
                    }
                }
            }
        }
        return match hit {
            Some(hit) => hit.color,
            None => scene.background_color,
        };
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
