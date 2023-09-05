use crate::helpers::is_power_of_2;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
    pub aa: usize,
}

#[allow(dead_code)]
impl Resolution {
    pub fn new(width: usize, height: usize, aa: usize) -> Result<Resolution, &'static str> {
        if width == 0 || height == 0 {
            return Err("width and height must be greater than 0");
        }
        if aa == 0 || !is_power_of_2(aa) {
            return Err("aa must be a power of 2");
        }
        return Ok(Resolution { width, height, aa });
    }
    pub fn print(self) {
        println!("{} {} {}", self.width, self.height, self.aa);
    }
}
