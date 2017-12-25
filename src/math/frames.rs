use math::{Matrix3, Vec3};

/*
 * Based on Quadroter Dyamics and Control from
 * http://rwbclasses.groups.et.byu.net/lib/exe/fetch.php?media=quadrotor:beardsquadrotornotes.pdf
 *
 * Intertial frame - earth fixed frame with origin at home location (North, East, Down)
 * Vehicle frame - Inertial centered on quadcopter
 * Vehicle-1 frame - Vehicle rotated by yaw
 * Vehicle-2 frame - Vehicle-1 rotated by pitch
 * Body frame - Vehicle-2 rotated by roll
 */

fn inertial_to_vehicle1(yaw: f64) -> Matrix3 {
    Matrix3::new(yaw.cos(), yaw.sin(), 0.0, -yaw.sin(), yaw.cos(), 0.0, 0.0, 0.0, 1.0)
}

fn vehicle1_to_vehicle2(pitch: f64) -> Matrix3 {
    Matrix3::new(pitch.cos(), 0.0, -pitch.sin(), 0.0, 1.0, 0.0, pitch.sin(), 0.0, pitch.cos())
}

fn vehicle2_to_body(roll: f64) -> Matrix3 {
    Matrix3::new(1.0, 0.0, 0.0, 0.0, roll.cos(), roll.sin(), 0.0, -roll.sin(), roll.cos())
}

fn inertial_to_body(pitch: f64, roll: f64, yaw: f64) -> Matrix3 {
    inertial_to_vehicle1(yaw) * vehicle1_to_vehicle2(pitch) * vehicle2_to_body(roll)
}

pub struct EulerAngles {
    pitch: f64,
    roll: f64,
    yaw: f64,
}

impl EulerAngles {
    pub fn new(pitch: f64, roll: f64, yaw: f64) -> Self {
        EulerAngles { pitch, roll, yaw }
    }

    // pub fn body_to_inertial(&self, xyz: Vec3) -> Vec3 {
    //     xyz / inertial_to_body(self.pitch, self.roll, self.yaw)
    // }

    pub fn inertial_to_body(&self, xyz: Vec3) -> Vec3 {
        inertial_to_body(self.pitch, self.roll, self.yaw) * xyz
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use math::Vec3;

    #[test]
    fn test_1z_inertial_to_vehicle1() {
        let original = Vec3::new(0.0, 0.0, 1.0);
        let expected = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(expected, inertial_to_vehicle1(45f64.to_radians()) * original);
    }
}
