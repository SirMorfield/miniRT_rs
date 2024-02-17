use crate::num::{is_power_of_2, PowerOf2};
use num_integer::Roots;
use std::num::NonZeroUsize;

#[derive(Clone, Copy)]
pub struct AALevel {
    aa: usize,
}

impl AALevel {
    pub fn new(aa: usize) -> Option<Self> {
        if aa == 1 {
            return Some(Self { aa });
        }
        if is_power_of_2(aa) {
            return Some(Self { aa });
        }
        return None;
    }
    pub fn get(&self) -> usize {
        self.aa
    }
    pub fn pixels_per_side(&self) -> usize {
        self.aa.sqrt() as usize
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Resolution {
    pub width: NonZeroUsize,
    pub height: NonZeroUsize,
    pub aa: AALevel,
    pub aspect_ratio: f32,
}

#[allow(dead_code)]
impl Resolution {
    pub fn new(width: NonZeroUsize, height: NonZeroUsize, aa: AALevel) -> Self {
        return Self {
            width,
            height,
            aa,
            aspect_ratio: width.get() as f32 / height.get() as f32,
        };
    }
    pub fn print(self) {
        println!("Resolution:");
        println!("  width    : {}", self.width.get());
        println!("  height   : {}", self.height.get());
        println!("  aa       : {}", self.aa.get());
        println!("  pixels   : {}", self.width.get() * self.height.get());
        println!(
            "  subpixels: {}",
            self.width.get() * self.height.get() * self.aa.get()
        );
    }
}
