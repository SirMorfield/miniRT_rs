use crate::vector::Vec3;

pub struct Ray {
    pub origin: Vec3<f32>,
    pub dir: Vec3<f32>,
}

impl Ray {
    pub fn new(origin: Vec3<f32>, dir: Vec3<f32>) -> Self {
        Self { origin, dir }
    }
}

#[derive(Clone, Copy)]
pub struct Hit {
    pub dist: f32,
    pub origin: Vec3<f32>,
    pub point: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub color: Vec3<u8>,
}
impl Hit {
    pub fn new(
        dist: f32,
        origin: Vec3<f32>,
        point: Vec3<f32>,
        normal: Vec3<f32>,
        color: Vec3<u8>,
    ) -> Self {
        Self {
            dist,
            origin,
            point,
            normal,
            color,
        }
    }
}

pub fn correct_normal(normal: Vec3<f32>, dir: &Vec3<f32>) -> Vec3<f32> {
    let inverse = normal * -1.0;
    return if normal.dot(&dir) < inverse.dot(&dir) {
        inverse
    } else {
        normal
    };
}

pub trait ToFixed {
    fn to_fixed(&self, precision: usize) -> String;
}

impl ToFixed for f32 {
    fn to_fixed(&self, precision: usize) -> String {
        let mut s = format!("{}", self);
        let mut dot = s.find('.');

        if precision == 0 {
            if let Some(x) = dot {
                s.truncate(x);
            }
            return s;
        }
        if let None = dot {
            dot = Some(s.len());
            s.push('.');
        }

        let mantissas = s.len() - dot.unwrap();
        for _ in mantissas..(precision + 1) {
            s.push('0');
        }
        s.truncate(dot.unwrap() + precision + 1);
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::util::ToFixed;

    #[test]
    fn to_fixed() {
        let n: f32 = 0.1234;
        assert_eq!(n.to_fixed(0), "0");
        assert_eq!(n.to_fixed(3), "0.123");
        assert_eq!(n.to_fixed(5), "0.12340");

        let n: f32 = 0.01;
        assert_eq!(n.to_fixed(0), "0");
        assert_eq!(n.to_fixed(5), "0.01000");

        let n: f32 = 100.0;
        assert_eq!(n.to_fixed(0), "100");
        assert_eq!(n.to_fixed(1), "100.0");
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct PositiveNonzeroF32(f32);

impl PositiveNonzeroF32 {
    pub fn new(value: f32) -> Option<Self> {
        match value {
            x if x <= 0.0 => None,
            _ => Some(PositiveNonzeroF32(value)),
        }
    }
    pub fn to_f32(&self) -> f32 {
        self.0
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Float0to1(f32);
impl Float0to1 {
    pub fn new(value: f32) -> Option<Self> {
        match value {
            x if x < 0.0 || x > 1.0 => None,
            _ => Some(Float0to1(value)),
        }
    }
    pub fn to_f32(&self) -> f32 {
        self.0
    }
}
