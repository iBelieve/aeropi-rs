use std::fmt;
use std::ops::{AddAssign,Sub,Div,Mul};

#[derive(Copy, Clone)]
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Coordinates {
    pub fn zero() -> Self {
        Coordinates {
            x: 0.0, y: 0.0, z: 0.0
        }
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Div<f32> for Coordinates {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Coordinates {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl Mul<f32> for Coordinates {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Coordinates {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Coordinates {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

pub fn twos_comp_combine(msb: u8, lsb: u8) -> i16 {
    let twos_comp: i32 = 256 * (msb as i32) + lsb as i32;

    if twos_comp >= 32768 {
        (twos_comp - 65536) as i16
    } else {
        twos_comp as i16
    }
}
