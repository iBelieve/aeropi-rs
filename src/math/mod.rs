mod frames;

use na::{self, Matrix, MatrixArray, U2, U3, Vector2, Vector3};

pub use self::frames::EulerAngles;

pub type Matrix2 = Matrix<f64, U2, U2, MatrixArray<f64, U2, U2>>;
pub type Matrix3 = Matrix<f64, U3, U3, MatrixArray<f64, U3, U3>>;
pub type Vec2 = Vector2<f64>;
pub type Vec3 = Vector3<f64>;

pub fn twos_comp_combine(msb: u8, lsb: u8) -> i16 {
    let twos_comp: i32 = 256 * (msb as i32) + lsb as i32;

    if twos_comp >= 32768 {
        (twos_comp - 65536) as i16
    } else {
        twos_comp as i16
    }
}

pub fn rotate_vector(x: f64, y: f64, angle: f64) -> (f64, f64) {
    // Based on https://en.wikipedia.org/wiki/Rotation_matrix
    let matrix = Matrix2::new(angle.cos(), -angle.sin(), angle.sin(), angle.cos());
    let rotated_vector = matrix * Vec2::new(x, y);

    (rotated_vector[(0, 0)], rotated_vector[(0, 1)])
}
