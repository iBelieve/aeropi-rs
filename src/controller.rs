use config::Calibration;
use motors::{self, Motor};
use sensors::Sensors;

pub struct FlightController {
    sensors: Sensors,
    motors: [Motor; 4]
}

impl FlightController {
    pub fn new() -> Self {
        FlightController {
            sensors: Sensors::new(),
            motors: motors::all()
        }
    }

    pub fn init(&mut self) {
        let calibration = Calibration::load();

        self.sensors.init(calibration.as_ref())
            .map(|calibration| calibration.save());

        println!("Flight controller initialized.");
    }

    pub fn calibrate(&mut self) {
        let calibration = self.sensors.calibrate();
        calibration.save();

        println!("Flight controller calibrated.");
    }

    fn update_motors(&mut self, pitch_out: i32, roll_out: i32, yaw_out: i32, vert_out: i32) {
        for motor in &mut self.motors {
            motor.update(pitch_out, roll_out, yaw_out, vert_out);
        }
    }
}
