use crate::camera::Camera;
use crate::light::Light;
use crate::resolution::Resolution;
use crate::scene::Scene;
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

    pub fn hit(&self, scene: &Scene, ray: &Ray) -> Option<Hit> {
        let mut hit: Option<Hit> = None;

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
        return hit;
    }

    // TODO: move to Hit
    pub fn get_color(&self, scene: &Scene, hit: &Hit) -> Vec3<u8> {
        let mut acc = Vec3::homogeneous(0 as u8);

        for light in &scene.lights {
            if !self.is_clear_path(scene, &hit.point, &light) {
                continue;
            }
            let relative = light.color * light.relative_intensity(&hit.point, &hit.normal);
            // todo this should be done better
            if (u8::MAX - acc.x) < relative.x {
                acc.x = u8::MAX;
            } else {
                acc.x += relative.x;
            }
            if (u8::MAX - acc.y) < relative.y {
                acc.y = u8::MAX;
            } else {
                acc.y += relative.y;
            }
            if (u8::MAX - acc.z) < relative.z {
                acc.z = u8::MAX;
            } else {
                acc.z += relative.z;
            }
        }
        return acc;
    }

    fn is_clear_path(&self, scene: &Scene, point: &Vec3<f32>, light: &Light) -> bool {
        let v: Vec3<f32> = (light.origin - *point).normalized();
        let to_light: Ray = Ray::new(*point, v);

        let hit = self.hit(&scene, &to_light);
        match &hit {
            None => true,
            Some(hit) => hit.dist * hit.dist > light.origin.distance2(point),
        }
    }

    pub fn render(&self, scene: &Scene, camera: &Camera, x: f32, y: f32) -> Vec3<u8> {
        let ray = self.ray_from_pixel(camera, x, y);
        let hit = self.hit(scene, &ray);

        return match hit {
            Some(hit) => self.get_color(scene, &hit),
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
