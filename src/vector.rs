#[derive(Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// implement add for Vec3
impl<T> std::ops::Add for Vec3<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

// implement sub for Vec3
impl<T> std::ops::Sub for Vec3<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        return Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

// implement mul for Vec3
impl<T> std::ops::Mul for Vec3<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        return Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        };
    }
}

// implement div for Vec3
impl<T> std::ops::Div for Vec3<T>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        return Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        };
    }
}

// implement add assign for Vec3
impl<T> std::ops::AddAssign for Vec3<T>
where
    T: std::ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// implement sub assign for Vec3
impl<T> std::ops::SubAssign for Vec3<T>
where
    T: std::ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// implement mul assign for Vec3
impl<T> std::ops::MulAssign for Vec3<T>
where
    T: std::ops::MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// implement div assign for Vec3
impl<T> std::ops::DivAssign for Vec3<T>
where
    T: std::ops::DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

// implement neg for Vec3
impl<T> std::ops::Neg for Vec3<T>
where
    T: std::ops::Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        return Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl std::ops::Mul<f32> for Vec3<f32> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        return Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

#[allow(dead_code)]
impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn homogeneous(xyz: T) -> Self
    where
        T: Clone,
    {
        Vec3 {
            x: xyz.clone(),
            y: xyz.clone(),
            z: xyz.clone(),
        }
    }

    pub fn unit(x: T, y: T, z: T) -> Option<Self>
    where
        T: num_traits::float::Float,
    {
        let v = Vec3::new(x, y, z);
        if !v.is_unit() {
            return None;
        }
        return Some(v);
    }

    pub fn to_unit(x: T, y: T, z: T) -> Self
    where
        T: std::ops::Div<Output = T>,
        T: std::ops::DivAssign,
        T: num_traits::float::Float,
    {
        let mut v = Vec3::new(x, y, z);
        return v.normalized();
    }

    pub fn from_vec3(other: &Vec3<T>) -> Self
    where
        T: Clone,
    {
        Vec3 {
            x: other.x.clone(),
            y: other.y.clone(),
            z: other.z.clone(),
        }
    }

    pub fn dot(self, other: &Vec3<T>) -> T
    where
        T: std::ops::Mul<Output = T>,
        T: std::ops::Add<Output = T>,
        T: Clone,
    {
        return self.x.clone() * other.x.clone()
            + self.y.clone() * other.y.clone()
            + self.z.clone() * other.z.clone();
    }

    pub fn cross(self, other: &Vec3<T>) -> Vec3<T>
    where
        T: std::ops::Mul<Output = T>,
        T: std::ops::Sub<Output = T>,
        T: Clone,
    {
        return Vec3::new(
            self.y.clone() * other.z.clone() - self.z.clone() * other.y.clone(),
            self.z.clone() * other.x.clone() - self.x.clone() * other.z.clone(),
            self.x.clone() * other.y.clone() - self.y.clone() * other.x.clone(),
        );
    }

    pub fn length2(self) -> T
    where
        T: std::ops::Mul<Output = T>,
        T: std::ops::Add<Output = T>,
        T: Clone,
    {
        return self.x.clone() * self.x.clone()
            + self.y.clone() * self.y.clone()
            + self.z.clone() * self.z.clone();
    }

    pub fn length(self) -> T
    where
        T: std::ops::Mul<Output = T>,
        T: std::ops::Add<Output = T>,
        T: std::clone::Clone,
        T: num_traits::float::Float,
    {
        return self.length2().sqrt();
    }

    pub fn is_unit(self) -> bool
    where
        T: num_traits::float::Float,
    {
        let len = self.length() - T::one(); // ?
        return len.abs() < T::epsilon();
    }

    pub fn normalize(&mut self)
    where
        T: std::ops::Div<Output = T>,
        T: std::ops::DivAssign,
        T: num_traits::float::Float,
    {
        // TODO
        let length = Vec3::new(self.x, self.y, self.z).length();
        self.x /= length.clone();
        self.y /= length.clone();
        self.z /= length.clone();
    }

    pub fn normalized(&mut self) -> Vec3<T>
    where
        T: std::ops::Div<Output = T>,
        T: std::ops::DivAssign,
        T: num_traits::float::Float,
    {
        // TODO
        let length = Vec3::new(self.x, self.y, self.z).length();
        self.x /= length.clone();
        self.y /= length.clone();
        self.z /= length.clone();

        // TODO: return self without copy?
        return Vec3::from_vec3(self);
    }

    pub fn translate(self, other: Vec3<T>, t: T) -> Vec3<T>
    where
        T: std::ops::Add<Output = T>,
        T: std::ops::Mul<Output = T>,
        T: Clone,
    {
        return Vec3::new(
            self.x.clone() + other.x * t.clone(),
            self.y.clone() + other.y * t.clone(),
            self.z.clone() + other.z * t.clone(),
        );
    }

    pub fn min(self, other: Vec3<T>) -> Vec3<T>
    where
        T: std::cmp::Ord,
    {
        return Vec3::new(
            std::cmp::min(self.x, other.x),
            std::cmp::min(self.y, other.y),
            std::cmp::min(self.z, other.z),
        );
    }

    pub fn max(self, other: Vec3<T>) -> Vec3<T>
    where
        T: std::cmp::Ord,
    {
        return Vec3::new(
            std::cmp::max(self.x, other.x),
            std::cmp::max(self.y, other.y),
            std::cmp::max(self.z, other.z),
        );
    }
}
