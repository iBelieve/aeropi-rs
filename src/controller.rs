use config::Calibration;
use math::{rotate_vector, EulerAngles, Vec3};
use motors::Motors;
use sensors::Sensors;

pub struct FlightController {
    sensors: Sensors,
    motors: Motors,
    target_velocity: Vec3
}

impl FlightController {
    pub fn new() -> Self {
        FlightController {
            sensors: Sensors::new(),
            motors: Motors::new(),
            target_velocity: Vec3::zero()
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

    pub fn step(&mut self) {
        let readings = self.sensors.read();
        let angles = EulerAngles::new(readings.pitch, readings.roll, readings.yaw);

        let quad_v = angles.inertial_to_body(self.target_velocity);
        let (aligned_vx, aligned_vy) = rotate_vector(self.target_velocity.x, self.target_velocity.y, readings.yaw);
    }
}
