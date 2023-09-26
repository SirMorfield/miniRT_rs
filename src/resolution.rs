use crate::num::PowerOf2;
use num_integer::Roots;
use std::num::NonZeroUsize;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Resolution {
    pub width: NonZeroUsize,
    pub height: NonZeroUsize,
    pub aa: PowerOf2,
}

#[allow(dead_code)]
impl Resolution {
    pub fn new(width: NonZeroUsize, height: NonZeroUsize, aa: PowerOf2) -> Self {
        return Self { width, height, aa };
    }
    pub fn pixels_per_side(&self) -> usize {
        return self.aa.get().sqrt() as usize;
    }
    pub fn print(self) {
        println!("{} {}", self.width, self.height);
    }
}
