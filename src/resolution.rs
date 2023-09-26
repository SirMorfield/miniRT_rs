use std::num::NonZeroUsize;
// use crate::helpers::is_power_of_2;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Resolution {
    pub width: NonZeroUsize,
    pub height: NonZeroUsize,
    // pub aa: usize,
}

#[allow(dead_code)]
impl Resolution {
    pub fn new(width: NonZeroUsize, height: NonZeroUsize) -> Self {
        return Self { width, height };
    }
    pub fn print(self) {
        println!("{} {}", self.width, self.height);
    }
}
