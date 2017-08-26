use config::Calibration;
use math::{EulerAngles, Vec3, rotate_vector};
use motors::Motors;
use pid_control::{Controller, PIDController};
use sensors::Sensors;

pub struct FlightController {
    sensors: Sensors,
    motors: Motors,
    target_velocity: Vec3,
    aligned_vx_pid: PIDController,
    aligned_vy_pid: PIDController,
    quad_vz_pid: PIDController,
}

impl FlightController {
    pub fn new() -> Self {
        FlightController {
            sensors: Sensors::new(),
            motors: Motors::new(),
            target_velocity: Vec3::zero(),
            // TODO: Pick the right constants for the three PIDs
            aligned_vx_pid: PIDController::new(1.0, 1.0, 1.0),
            aligned_vy_pid: PIDController::new(1.0, 1.0, 1.0),
            quad_vz_pid: PIDController::new(1.0, 1.0, 1.0),
        }
    }

    pub fn init(&mut self) {
        let calibration = Calibration::load();

        self.sensors.init(calibration.as_ref())
            .map(|calibration| calibration.save());

        self.motors.init();

        println!("Flight controller initialized.");
    }

    pub fn calibrate(&mut self) {
        let calibration = self.sensors.calibrate();
        calibration.save();

        println!("Flight controller calibrated.");
    }

    #[allow(dead_code)] // TODO: Remove once runloop is implement
    pub fn step(&mut self) {
        let dt: f64 = 0.01; // TODO: Properly calculate this
        let readings = self.sensors.read();
        let angles = EulerAngles::new(readings.pitch, readings.roll, readings.yaw);

        let quad_v = angles.inertial_to_body(self.target_velocity);
        let (aligned_vx, aligned_vy) =
            rotate_vector(self.target_velocity.x, self.target_velocity.y, readings.yaw);

        // Calculate the xyz acceleration based on the requested/current xyz velocity
        // Notes:
        //  - The x/y acceleration is "aligned", or rotated so x always points forward and y always points right. This
        //    is so the aligned velocity/acceleration is always in the same frame as pitch and roll
        //  - The vertical acceleration is in the quad frame because we'll be controlling the acceleration
        //    directly up and down (by changing the power of the props), which are in the quad frame.
        self.aligned_vx_pid.set_target(aligned_vx);
        self.aligned_vy_pid.set_target(aligned_vy);
        self.quad_vz_pid.set_target(quad_v.z);

        let aligned_accel_x = self.aligned_vx_pid.update(readings.aligned_vx, dt);
        let aligned_accel_y = self.aligned_vy_pid.update(readings.aligned_vy, dt);
        let quad_accel_z = self.quad_vz_pid.update(readings.quad_vz, dt);
    }
}
