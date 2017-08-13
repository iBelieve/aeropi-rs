use config::Calibration;
use motors::Motors;
use sensors::Sensors;

pub struct FlightController {
    sensors: Sensors,
    motors: Motors
}

impl FlightController {
    pub fn new() -> Self {
        FlightController {
            sensors: Sensors::new(),
            motors: Motors::new()
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
}
