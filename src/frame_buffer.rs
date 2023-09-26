use crate::resolution::Resolution;
use crate::util::Float0to1;
use crate::vector::Vec3;
use bmp::{Image, Pixel};
use std::io;
use std::vec::Vec;

#[allow(dead_code)]
pub struct FrameBuffer {
    buffer: Vec<Vec3<u8>>,
    resolution: Resolution,
    pixel_index: usize,
}

#[allow(dead_code)]
impl FrameBuffer {
    // ? why am I allowed to use a result when there is only ever a ok?
    pub fn new(resolution: Resolution) -> Result<FrameBuffer, &'static str> {
        let mut buffer = Vec::<Vec3<u8>>::new();
        buffer.resize(
            resolution.width * resolution.height,
            Vec3::<u8>::homogeneous(0),
        );
        return Ok(FrameBuffer {
            buffer,
            resolution,
            pixel_index: 0,
        });
    }

    // returns 0 - 1
    pub fn progress(&self) -> Float0to1 {
        return Float0to1::new(
            self.pixel_index as f32 / (self.resolution.width * self.resolution.height) as f32,
        )
        .unwrap_or(Float0to1::new(0.0).unwrap());
    }

    pub fn get_coordinate(&mut self) -> Option<(usize, usize)> {
        if self.pixel_index >= self.buffer.len() {
            return None;
        }
        let pix = (
            self.pixel_index % self.resolution.width,
            self.pixel_index / self.resolution.width,
        );
        self.pixel_index += 1;
        return Some(pix);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3<u8>) {
        let i = x + y * self.resolution.width;
        if i >= self.buffer.len() {
            // TODO: error handling
            return;
        }
        self.buffer[i] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Vec3<u8>> {
        let i = x + y * self.resolution.width;
        if i >= self.buffer.len() {
            return None;
        }
        return Some(self.buffer[i]);
    }

    pub fn save_as_bmp(self, path: &std::path::Path) -> io::Result<()> {
        let mut img = Image::new(self.resolution.width as u32, self.resolution.height as u32);

        for x in 0..self.resolution.width {
            for y in 0..self.resolution.height {
                let color = self.get_pixel(x, y).unwrap();
                img.set_pixel(x as u32, y as u32, Pixel::new(color.x, color.y, color.z));
            }
        }

        return img.save(path);
    }
}
