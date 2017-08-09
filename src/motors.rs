/*
 * The quadcopter will be powered by four motors + propellers. The propellers will be configured
 * such that there are two clockwise and two counter-clockwise propellers configured on opposite
 * diagonals to balance out the rotation and keep the quadcopter steady in flight. This module
 * provides control of those motors by exposing a single public function, update, which takes
 * pitch, roll, yaw, and vertical adjustments and updates the motor ESCs.
 */

#[derive(PartialEq)]
enum MotorLocation {
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight
}

pub struct Motor {
    location: MotorLocation
}

impl Motor {
    fn new(location: MotorLocation) -> Self {
        Motor { location }
    }

    fn is_front(&self) -> bool {
        self.location == MotorLocation::FrontLeft || self.location == MotorLocation::FrontRight
    }

    fn is_left(&self) -> bool {
        self.location == MotorLocation::FrontLeft || self.location == MotorLocation::BackLeft
    }

    fn is_cw(&self) -> bool {
        // TODO: Update as necessary
        self.location == MotorLocation::FrontRight || self.location == MotorLocation::BackLeft
    }

    pub fn update(&mut self, pitch_out: i32, roll_out: i32, yaw_out: i32, vert_out: i32) {
        let mut delta_spin = vert_out;

        // For a left downwards roll, the x gyro goes negative, so the PID error is positive,
        // meaning PID output is positive, meaning this needs to be added to the left blades
        // and subtracted from the right.
        delta_spin += if self.is_left() { roll_out } else { -roll_out };

        // For a forward downwards pitch, the y gyro goes positive The PID error is negative as a
        // result, meaning PID output is negative, meaning this needs to be subtracted from the
        // front blades and added to the back.
        delta_spin += if self.is_front() { -pitch_out } else { pitch_out };

        // For CW yaw, the z gyro goes negative, so the PID error is postitive, meaning PID
        // output is positive, meaning this need to be added to the ACW (FL and BR) blades and
        // subtracted from the CW (FR & BL) blades.
        delta_spin += if self.is_cw() { -yaw_out } else { yaw_out };

        // TODO: Use the delta spin to update the ESCs
    }
}

pub fn all() -> [Motor; 4] {
    [
        Motor::new(MotorLocation::FrontLeft),
        Motor::new(MotorLocation::FrontRight),
        Motor::new(MotorLocation::BackLeft),
        Motor::new(MotorLocation::BackRight)
    ]
}
