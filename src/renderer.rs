use crate::camera::Camera;
use crate::light::Light;
use crate::resolution::Resolution;
use crate::scene_readers::Scene;
use crate::util::{Hit, PixelReqBuffer, PixelRes, PixelResBuffer, Ray, PIXEL_BUFFER_SIZE};
use crate::vector::Point;
use std::num::NonZeroUsize;
use std::sync::{mpsc, Arc, Mutex, RwLock};

#[derive(Clone)]
pub struct Renderer {
    resolution: Resolution,
}

impl Renderer {
    pub fn new(resolution: Resolution) -> Self {
        Self { resolution }
    }

    /// TODO: this is completely broken when the fov changes
    pub fn ray_from_pixel(&self, camera: &Camera, x: f32, y: f32) -> Ray {
        let px = (2.0 * x / (self.resolution.width.get() as f32) - 1.0) * self.resolution.aspect_ratio * camera.fow_tan;
        let py = (2.0 * y / (self.resolution.height.get() as f32) - 1.0) * camera.fow_tan;

        let positive_x: Point<f32> = if camera.dir.x == 0.0 && camera.dir.z == 0.0 {
            Point::new(1.0, 0.0, 0.0)
        } else {
            camera.dir.cross(&Point::new(0.0, 1.0, 0.0))
        };

        let negative_y = camera.dir.cross(&positive_x);
        let mut ray_dir = ((positive_x * px) + (negative_y * py)) + camera.dir;
        ray_dir.normalize();

        return Ray::new(camera.pos, ray_dir);
    }

    pub fn hit(&self, scene: &Scene, ray: &Ray) -> Option<Hit> {
        return scene.triangles.hit(&ray);
    }

    // TODO: move to Hit
    pub fn get_color(&self, scene: &Scene, hit: &Hit) -> Point<u8> {
        let color = Point::new(hit.color.x as f32, hit.color.y as f32, hit.color.z as f32);
        let mut acc = Point::homogeneous(0.0);
        let mut additions: usize = 0;

        for light in &scene.lights {
            if !self.is_clear_path(scene, &hit.point, &light) {
                continue;
            }

            let relative = color * light.relative_intensity(&hit.point, &hit.normal);
            acc += relative;
            additions += 1;
        }
        acc /= additions as f32;
        return Point::new(acc.x as u8, acc.y as u8, acc.z as u8);
    }

    fn is_clear_path(&self, scene: &Scene, point: &Point<f32>, light: &Light) -> bool {
        let v = (light.origin - *point).to_normalized();
        let to_light = Ray::new(*point, v);

        let hit = self.hit(&scene, &to_light);
        match &hit {
            None => true,
            Some(hit) => hit.dist * hit.dist > light.origin.distance2(point),
        }
    }

    fn average_color(colors: &Vec<Point<u8>>) -> Point<u8> {
        let mut final_color = Point::<u64>::homogeneous(0);
        for color in colors {
            final_color.x += color.x as u64;
            final_color.y += color.y as u64;
            final_color.z += color.z as u64;
        }

        final_color.x /= colors.len() as u64;
        final_color.y /= colors.len() as u64;
        final_color.z /= colors.len() as u64;

        let final_color = Point::new(final_color.x as u8, final_color.y as u8, final_color.z as u8);
        return final_color;
    }

    pub fn render(&self, scene: &Scene, camera: &Camera, x: f32, y: f32) -> Point<u8> {
        let mut colors: Vec<Point<u8>> = Vec::new();
        colors.reserve(self.resolution.aa.get());
        let row_columns = self.resolution.aa.pixels_per_side();

        for sub_y in 0..row_columns {
            for sub_x in 0..row_columns {
                let x = x + (sub_x as f32) / (row_columns as f32);
                let y = y + (sub_y as f32) / (row_columns as f32);
                let ray = self.ray_from_pixel(camera, x, y);
                let hit = self.hit(scene, &ray);
                match hit {
                    Some(hit) => colors.push(self.get_color(scene, &hit)),
                    None => (),
                }
            }
        }
        if colors.len() == 0 {
            return scene.void();
        }
        return Self::average_color(&colors) + scene.ambient.absolute_color();
    }
}

pub fn render_multithreaded(
    scene: Arc<RwLock<Scene>>,
    resolution: &Resolution,
    pixels: impl Iterator<Item = PixelReqBuffer> + Send + 'static,
) -> impl Iterator<Item = PixelResBuffer> {
    let threads = std::thread::available_parallelism()
        .unwrap_or(NonZeroUsize::new(8).unwrap())
        .get();
    let (tx, rx) = mpsc::channel();
    let renderer = Arc::new(Renderer::new(resolution.clone()));
    let pixels = Arc::new(Mutex::new(pixels));

    for _ in 0..threads {
        let tx = tx.clone();
        let renderer = renderer.clone();
        let scene = scene.clone();
        let pixels_clone = pixels.clone();

        std::thread::spawn(move || {
            let scene = scene.read().unwrap();
            loop {
                let mut buffers = pixels_clone.lock().unwrap();
                let buffer = buffers.next();
                if buffer.is_none() {
                    return;
                }
                let buffer = buffer.unwrap();
                drop(buffers); // unlock mutex as soon as possible

                let mut colors = [None; PIXEL_BUFFER_SIZE];
                for (i, pixel) in buffer.into_iter().enumerate() {
                    if let Some(pixel) = pixel {
                        let color = renderer.render(&scene, &scene.camera, pixel.x as f32, pixel.y as f32);
                        colors[i] = Some(PixelRes::new(pixel.x, pixel.y, color));
                    }
                }
                if colors.iter().all(|c| c.is_none()) {
                    return;
                }
                tx.send(colors).unwrap();
            }
        });
    }
    drop(tx);
    rx.into_iter()
}
