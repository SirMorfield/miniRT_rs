use crate::num::Float0to1;
use crate::random_iterator::RandomIterator;
use crate::resolution::Resolution;
use crate::util::{PixelReq, PixelReqBuffer, PixelResBuffer, PIXEL_BUFFER_SIZE};
use crate::vector::Point;
use bitvec::prelude::*;
use bmp::{Image, Pixel};
use std::io;
use std::vec::Vec;

pub fn to_u32(color: Point<u8>) -> u32 {
    (color.z as u32) | ((color.y as u32) << 8) | ((color.x as u32) << 16)
}

pub fn to_u8(color: u32) -> Point<u8> {
    Point::new(
        ((color >> 16) & 0xFF) as u8,
        ((color >> 8) & 0xFF) as u8,
        (color & 0xFF) as u8,
    )
}

pub struct PixelProvider {
    pixel_index: RandomIterator,
    resolution: Resolution,
}

impl PixelProvider {
    pub fn new(resolution: &Resolution) -> PixelProvider {
        PixelProvider {
            pixel_index: RandomIterator::new(resolution.width.get() * resolution.height.get()),
            resolution: *resolution,
        }
    }

    pub fn reset(&mut self) {
        self.pixel_index.reset();
    }

    #[allow(dead_code)]
    pub fn get_coordinate(&mut self) -> Option<PixelReq> {
        match self.pixel_index.next() {
            None => None,
            Some(i) => Some(self.i_to_coord(i)),
        }
    }

    pub fn get_coordinates(&mut self) -> PixelReqBuffer {
        let mut result = [None; PIXEL_BUFFER_SIZE];
        for i in 0..PIXEL_BUFFER_SIZE {
            result[i] = match self.pixel_index.next() {
                None => None,
                Some(i) => Some(self.i_to_coord(i)),
            };
        }
        result
    }

    pub fn get_coordinate_iter<'a>(&'a mut self) -> impl Iterator<Item = PixelReq> + 'a {
        std::iter::from_fn(move || match self.pixel_index.next() {
            None => None,
            Some(i) => Some(self.i_to_coord(i)),
        })
    }

    fn i_to_coord(&self, i: usize) -> PixelReq {
        PixelReq::new(i % self.resolution.width.get(), i / self.resolution.width.get())
    }
}

impl Iterator for PixelProvider {
    type Item = PixelReqBuffer;

    fn next(&mut self) -> Option<PixelReqBuffer> {
        Some(self.get_coordinates())
    }
}

#[allow(dead_code)]
pub enum Flip {
    Horizontal,
    Vertical,
}

pub struct FrameBuffer {
    buffer: Vec<u32>,
    assigned_pixels: BitVec<u32, Lsb0>,
    resolution: Resolution,
}

impl FrameBuffer {
    // ? why am I allowed to use a result when there is only ever a ok?
    pub fn new(resolution: &Resolution) -> Result<FrameBuffer, &'static str> {
        let mut buffer = Vec::<u32>::new();
        buffer.resize(resolution.width.get() * resolution.height.get(), 0);
        return Ok(FrameBuffer {
            buffer,
            resolution: *resolution,
            assigned_pixels: bitvec![u32, Lsb0; 0; resolution.width.get() * resolution.height.get()],
        });
    }

    pub fn pixel_count(&self) -> usize {
        return self.resolution.width.get() * self.resolution.height.get();
    }

    pub fn progress(&self) -> Float0to1 {
        let assigned = self.assigned_pixels.iter().filter(|x| **x).count();
        return Float0to1::new(assigned as f32 / self.pixel_count() as f32).unwrap();
    }

    pub fn is_complete(&self) -> bool {
        self.assigned_pixels.iter().all(|x| *x)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Point<u8>) {
        let i = x + y * self.resolution.width.get();
        if i >= self.buffer.len() {
            panic!("Index out of bounds");
        }
        self.buffer[i] = to_u32(color);
        self.assigned_pixels.set(i, true);
    }

    pub fn set_pixel_from_buffer(&mut self, buffer: &PixelResBuffer) {
        for pixel in buffer {
            if let Some(pixel) = pixel {
                self.set_pixel(pixel.x, pixel.y, pixel.color);
            }
        }
    }

    pub fn set_pixel_from_iterator(&mut self, iter: &mut impl Iterator<Item = PixelResBuffer>) {
        for pixel_buffer in iter {
            for pixel in pixel_buffer {
                if let Some(pixel) = pixel {
                    self.set_pixel(pixel.x, pixel.y, pixel.color);
                }
            }
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Point<u8>> {
        let i = x + y * self.resolution.width.get();
        if i >= self.buffer.len() {
            return None;
        }
        return Some(to_u8(self.buffer[i]));
    }

    fn i_to_coord(&self, i: usize) -> PixelReq {
        PixelReq::new(i % self.resolution.width.get(), i / self.resolution.width.get())
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

    #[allow(dead_code)]
    pub fn save_as_bmp(&self, path: &std::path::Path) -> io::Result<()> {
        let mut img = Image::new(self.resolution.width.get() as u32, self.resolution.height.get() as u32);

        for x in 0..self.resolution.width.get() {
            for y in 0..self.resolution.height.get() {
                let color = self.get_pixel(x, y).unwrap();
                img.set_pixel(x as u32, y as u32, Pixel::new(color.x, color.y, color.z));
            }
        }

        return img.save(path);
    }
}
