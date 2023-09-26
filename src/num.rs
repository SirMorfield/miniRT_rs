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
    use crate::num::ToFixed;

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
    pub fn get(&self) -> f32 {
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
    pub fn get(&self) -> f32 {
        self.0
    }
}
