use crate::num::Float0to1;
use crate::resolution::Resolution;
use crate::vector::Vec3;
use bmp::{Image, Pixel};
use std::io;
use std::vec::Vec;
#[allow(dead_code)]
pub enum Flip {
    Horizontal,
    Vertical,
}

pub struct FrameBuffer {
    buffer: Vec<Vec3<u8>>,
    resolution: Resolution,
    pixel_index: usize,
}

impl FrameBuffer {
    // ? why am I allowed to use a result when there is only ever a ok?
    pub fn new(resolution: Resolution) -> Result<FrameBuffer, &'static str> {
        let mut buffer = Vec::<Vec3<u8>>::new();
        buffer.resize(
            resolution.width.get() * resolution.height.get(),
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
            self.pixel_index as f32
                / (self.resolution.width.get() * self.resolution.height.get()) as f32,
        )
        .unwrap_or(Float0to1::new(0.0).unwrap());
    }

    pub fn get_coordinate(&mut self) -> Option<(usize, usize)> {
        if self.pixel_index >= self.buffer.len() {
            return None;
        }
        let pix = self.i_to_coord(self.pixel_index);
        self.pixel_index += 1;
        return Some(pix);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Vec3<u8>) {
        let i = x + y * self.resolution.width.get();
        if i >= self.buffer.len() {
            // TODO: error handling
            return;
        }
        self.buffer[i] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Vec3<u8>> {
        let i = x + y * self.resolution.width.get();
        if i >= self.buffer.len() {
            return None;
        }
        return Some(self.buffer[i]);
    }

    fn i_to_coord(&self, i: usize) -> (usize, usize) {
        (
            i % self.resolution.width.get(),
            i / self.resolution.width.get(),
        )
    }
    fn coord_to_i(&self, x: usize, y: usize) -> usize {
        y * self.resolution.width.get() + x
    }

    #[allow(dead_code)]
    pub fn flip(&mut self, direction: Flip) {
        let width = self.resolution.width.get();
        let height = self.resolution.height.get();
        match direction {
            Flip::Horizontal => {
                for y in 0..height / 2 {
                    for x in 0..width {
                        let top = self.coord_to_i(x, y);
                        let bottom = self.coord_to_i(x, height - y - 1);
                        self.buffer.swap(top, bottom);
                    }
                }
            }
            Flip::Vertical => {
                for x in 0..width / 2 {
                    for y in 0..height {
                        let left = self.coord_to_i(x, y);
                        let right = self.coord_to_i(width - x - 1, y);
                        self.buffer.swap(left, right);
                    }
                }
            }
        }
    }

    pub fn save_as_bmp(&self, path: &std::path::Path) -> io::Result<()> {
        let mut img = Image::new(
            self.resolution.width.get() as u32,
            self.resolution.height.get() as u32,
        );

        for x in 0..self.resolution.width.get() {
            for y in 0..self.resolution.height.get() {
                let color = self.get_pixel(x, y).unwrap();
                img.set_pixel(x as u32, y as u32, Pixel::new(color.x, color.y, color.z));
            }
        }

        return img.save(path);
    }
}
