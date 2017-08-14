use std::fmt;
use std::ops::{AddAssign,Sub,Div,Mul};
use ndarray::Array2;
use num::Num;

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vec3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Num> Vec3<T> {
    pub fn zero() -> Self {
        Vec3 {
            x: T::zero(), y: T::zero(), z: T::zero()
        }
    }
}

impl<T: Num + Copy> Vec3<T> {
    pub fn as_matrix(&self) -> Array2<T> {
        array![[self.x, self.y, self.z]]
    }
}

impl<T: Num + fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: Num + AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Num + Copy> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl<T: Num + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl<T: Num> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

// Matrix math

impl<T: Num> From<Array2<T>> for Vec3<T> {
    fn from(matrix: Array2<T>) -> Self {
        Vec3::zero()
    }
}

impl<T: Num + Copy> Mul<Array2<T>> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: Array2<T>) -> Self {
        Vec3::from(self.as_matrix() * rhs)
    }
}

impl<T: Num + Copy> Div<Array2<T>> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: Array2<T>) -> Self {
        Vec3::from(self.as_matrix() / rhs)
    }
}
