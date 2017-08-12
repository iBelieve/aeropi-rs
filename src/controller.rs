use std::fs::File;
use config::Calibration;
use motors::{self, Motor};
use sensors::Sensors;
use serde_yaml;

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
        let calibration = load_calibration();

        self.sensors.init(calibration.as_ref())
            .map(|c| save_calibration(&c));

        println!("Flight controller initialized.");
    }

    pub fn calibrate(&mut self) {
        let calibration = self.sensors.calibrate();
        save_calibration(&calibration);

        println!("Flight controller calibrated.");
    }

    fn update_motors(&mut self, pitch_out: i32, roll_out: i32, yaw_out: i32, vert_out: i32) {
        for motor in &mut self.motors {
            motor.update(pitch_out, roll_out, yaw_out, vert_out);
        }
    }
}

fn load_calibration() -> Option<Calibration> {
    File::open("calibration.yml").ok()
        .and_then(|file| serde_yaml::from_reader(&file).ok())
}

fn save_calibration(calibration: &Calibration) {
    let file = File::create("calibration.yml")
        .expect("Unable to create calibration file");
    serde_yaml::to_writer(&file, calibration)
        .expect("Unable to save calibration data");
    println!("Saved updated calibration.");
}
