#[derive(Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! impl_overload {
    ($Op:ident $Fn:ident $OpSymbol:tt) => {
        impl<T> std::ops::$Op for Vec3<T>
        where
            T: std::ops::$Op<Output = T>,
        {
            type Output = Self;
            fn $Fn(self, rhs: Self) -> Self {
				return Vec3 {
					x: self.x $OpSymbol rhs.x,
					y: self.y $OpSymbol rhs.y,
					z: self.z $OpSymbol rhs.z,
				};
			}
        }
    };
}

macro_rules! impl_overload_assign {
	($Op:ident $Fn:ident $OpSymbol:tt) => {
		impl<T> std::ops::$Op for Vec3<T>
		where
			T: std::ops::$Op,
		{
			fn $Fn(&mut self, rhs: Self) {
				self.x $OpSymbol rhs.x;
				self.y $OpSymbol rhs.y;
				self.z $OpSymbol rhs.z;
			}
		}
	};
}

impl_overload!(Add add +);
impl_overload!(Sub sub -);
impl_overload!(Mul mul *);
impl_overload!(Div div /);
impl_overload_assign!(AddAssign add_assign +=);
impl_overload_assign!(SubAssign sub_assign -=);
impl_overload_assign!(MulAssign mul_assign *=);
impl_overload_assign!(DivAssign div_assign /=);

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

impl std::ops::Div<f32> for Vec3<f32> {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        return Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

impl std::ops::Mul<f32> for Vec3<u8> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        return Vec3 {
            x: (self.x as f32 * rhs) as u8,
            y: (self.y as f32 * rhs) as u8,
            z: (self.z as f32 * rhs) as u8,
        };
    }
}

#[allow(dead_code)]
impl<T> Vec3<T>
where
    T: Clone + Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn homogeneous(xyz: T) -> Self
    where
        T: Clone,
    {
        Vec3 {
            x: xyz,
            y: xyz,
            z: xyz,
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
        v.normalize();
        return v;
    }

    pub fn from_vec3(other: &Vec3<T>) -> Self
    where
        T: Clone,
    {
        Vec3 {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }

    pub fn dot(self, other: &Vec3<T>) -> T
    where
        T: std::ops::Mul<Output = T>,
        T: std::ops::Add<Output = T>,
        T: Clone,
    {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(self, other: &Vec3<T>) -> Vec3<T>
    where
        T: std::ops::Mul<Output = T>,
        T: std::ops::Sub<Output = T>,
        T: Clone,
    {
        return Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        );
    }

    pub fn length2(self) -> T
    where
        T: std::ops::Mul<Output = T>,
        T: std::ops::Add<Output = T>,
        T: Clone,
    {
        return self.x * self.x + self.y * self.y + self.z * self.z;
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
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn to_normalized(&self) -> Vec3<T>
    where
        T: std::ops::Div<Output = T>,
        T: std::ops::DivAssign,
        T: num_traits::float::Float,
    {
        let mut new = Vec3::new(self.x, self.y, self.z);
        new.normalize();
        return new;
    }

    pub fn translate(self, other: Vec3<T>, t: T) -> Vec3<T>
    where
        T: std::ops::Add<Output = T>,
        T: std::ops::Mul<Output = T>,
        T: Clone,
    {
        return Vec3::new(
            self.x + other.x * t,
            self.y + other.y * t,
            self.z + other.z * t,
        );
    }

    pub fn distance2(&self, other: &Vec3<T>) -> T
    where
        T: Clone,
        T: std::ops::Sub<Output = T>,
        T: std::ops::Add<Output = T>,
        T: std::ops::Mul<Output = T>,
    {
        return (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z);
    }

    pub fn distance(&self, other: &Vec3<T>) -> T
    where
        T: Clone,
        T: std::ops::Sub<Output = T>,
        T: std::ops::Add<Output = T>,
        T: std::ops::Mul<Output = T>,
        T: num_traits::float::Float,
    {
        return self.distance2(other).sqrt();
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
