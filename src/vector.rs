use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num_traits::{float::FloatCore, Float};

#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! impl_overload {
    ($Op:ident $Fn:ident $OpSymbol:tt) => {
        impl<T> $Op for Point<T>
        where
            T: $Op<Output = T>,
        {
            type Output = Self;
            fn $Fn(self, rhs: Self) -> Self {
				return Point {
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
		impl<T> $Op for Point<T>
		where
			T: $Op,
		{
			fn $Fn(&mut self, rhs: Self) {
				self.x $OpSymbol rhs.x;
				self.y $OpSymbol rhs.y;
				self.z $OpSymbol rhs.z;
			}
		}
	};
}

macro_rules! impl_overload_rhs {
	($Op:ident $Fn:ident $OpSymbol:tt) => {
		impl<T: $Op<Output = T>> $Op<T> for Point<T>
		where
    		T: Clone + Copy,
	{
			type Output = Self;
			fn $Fn(self, rhs: T) -> Self {
				return Point {
					x: self.x $OpSymbol rhs,
					y: self.y $OpSymbol rhs,
					z: self.z $OpSymbol rhs,
				};
			}
		}
	};
}
macro_rules! impl_overload_rhs_assign {
	($Op:ident $Fn:ident $OpSymbol:tt) => {
		impl<T: $Op> $Op<T> for Point<T>
		where
			T: Clone + Copy,
	{
			fn $Fn(&mut self, rhs: T) {
				self.x $OpSymbol rhs;
				self.y $OpSymbol rhs;
				self.z $OpSymbol rhs;
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
impl_overload_rhs!(Add add +);
impl_overload_rhs!(Sub sub -);
impl_overload_rhs!(Mul mul *);
impl_overload_rhs!(Div div /);
impl_overload_rhs_assign!(AddAssign add_assign +=);
impl_overload_rhs_assign!(SubAssign sub_assign -=);
impl_overload_rhs_assign!(MulAssign mul_assign *=);
impl_overload_rhs_assign!(DivAssign div_assign /=);

impl<T> Neg for Point<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        return Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: Eq> Eq for Point<T> {}

#[allow(dead_code)]
impl<T> Point<T>
where
    T: Clone + Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Point { x, y, z }
    }

    pub fn homogeneous(xyz: T) -> Self
    where
        T: Clone,
    {
        Point {
            x: xyz,
            y: xyz,
            z: xyz,
        }
    }

    pub fn unit(x: T, y: T, z: T) -> Option<Self>
    where
        T: num_traits::float::Float,
    {
        let v = Point::new(x, y, z);
        if !v.is_unit() {
            return None;
        }
        return Some(v);
    }

    pub fn to_unit(x: T, y: T, z: T) -> Self
    where
        T: Div<Output = T>,
        T: DivAssign,
        T: num_traits::float::Float,
    {
        let mut v = Point::new(x, y, z);
        v.normalize();
        return v;
    }

    pub fn from_Point(other: &Point<T>) -> Self
    where
        T: Clone,
    {
        Point {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }

    pub fn dot(self, other: &Point<T>) -> T
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: Clone,
    {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(self, other: &Point<T>) -> Point<T>
    where
        T: Mul<Output = T>,
        T: Sub<Output = T>,
        T: Clone,
    {
        return Point::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        );
    }

    pub fn length2(self) -> T
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: Clone,
    {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(self) -> T
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
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
        T: Div<Output = T>,
        T: DivAssign,
        T: num_traits::float::Float,
    {
        // TODO
        let length = Point::new(self.x, self.y, self.z).length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn to_normalized(&self) -> Point<T>
    where
        T: Div<Output = T>,
        T: DivAssign,
        T: num_traits::float::Float,
    {
        let mut new = Point::new(self.x, self.y, self.z);
        new.normalize();
        return new;
    }

    pub fn translate(self, other: Point<T>, t: T) -> Point<T>
    where
        T: Add<Output = T>,
        T: Mul<Output = T>,
        T: Clone,
    {
        return Point::new(
            self.x + other.x * t,
            self.y + other.y * t,
            self.z + other.z * t,
        );
    }

    pub fn distance2(&self, other: &Point<T>) -> T
    where
        T: Clone,
        T: Sub<Output = T>,
        T: Add<Output = T>,
        T: Mul<Output = T>,
    {
        return (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z);
    }

    pub fn distance(&self, other: &Point<T>) -> T
    where
        T: Clone,
        T: Sub<Output = T>,
        T: Add<Output = T>,
        T: Mul<Output = T>,
        T: num_traits::float::Float,
    {
        return self.distance2(other).sqrt();
    }

    pub fn min(self, other: Point<T>) -> Point<T>
    where
        T: std::cmp::Ord,
    {
        return Point::new(
            std::cmp::min(self.x, other.x),
            std::cmp::min(self.y, other.y),
            std::cmp::min(self.z, other.z),
        );
    }

    pub fn min_unsafe(self, other: Point<T>) -> Point<T>
    where
        T: std::cmp::PartialOrd,
    {
        return Point::new(
            std::cmp::min_by(self.x, other.x, |a, b| {
                a.partial_cmp(&b).unwrap_or(Ordering::Greater)
            }),
            std::cmp::min_by(self.y, other.y, |a, b| {
                a.partial_cmp(&b).unwrap_or(Ordering::Greater)
            }),
            std::cmp::min_by(self.z, other.z, |a, b| {
                a.partial_cmp(&b).unwrap_or(Ordering::Greater)
            }),
        );
    }

    pub fn max(self, other: Point<T>) -> Point<T>
    where
        T: std::cmp::Ord,
    {
        return Point::new(
            std::cmp::max(self.x, other.x),
            std::cmp::max(self.y, other.y),
            std::cmp::max(self.z, other.z),
        );
    }

    pub fn max_unsafe(self, other: Point<T>) -> Point<T>
    where
        T: std::cmp::PartialOrd,
    {
        return Point::new(
            std::cmp::max_by(self.x, other.x, |a, b| {
                a.partial_cmp(&b).unwrap_or(Ordering::Greater)
            }),
            std::cmp::max_by(self.y, other.y, |a, b| {
                a.partial_cmp(&b).unwrap_or(Ordering::Greater)
            }),
            std::cmp::max_by(self.z, other.z, |a, b| {
                a.partial_cmp(&b).unwrap_or(Ordering::Greater)
            }),
        );
    }

    pub fn is_finite(&self) -> bool
    where
        T: Float + FloatCore,
    {
        return Float::is_finite(self.x) && Float::is_finite(self.y) && Float::is_finite(self.z);
    }

    pub fn approx_eq(&self, other: &Self, epsilon: T) -> bool
    where
        T: Float + FloatCore,
    {
        return Float::abs(self.x - other.x) < epsilon
            && Float::abs(self.y - other.y) < epsilon
            && Float::abs(self.z - other.z) < epsilon;
    }
}
