mod accelerometer;

use self::accelerometer::Accelerometer;

pub struct Sensors {
    accelerometer: Accelerometer
}

impl Sensors {
    pub fn new() -> Self {
        Sensors {
            accelerometer: Accelerometer::new()
        }
    }

    pub fn init(&mut self) {
        self.accelerometer.enable();
        self.accelerometer.load_calibration();
        println!("Sensors initialized.");
    }

    pub fn calibrate(&mut self) {
        self.accelerometer.enable();
        self.accelerometer.calibrate();
        println!("Sensors calibrated.");
    }
}
