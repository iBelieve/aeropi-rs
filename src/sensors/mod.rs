mod accelerometer;

use config::Calibration;
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

    pub fn init(&mut self, calibration: Option<&Calibration>) -> Option<Calibration> {
        self.accelerometer.enable();

        if let Some(calibration) = calibration {
            self.accelerometer.set_calibration(&calibration.accelerometer);
            None
        } else {
            Some(Calibration {
                accelerometer: self.accelerometer.calibrate()
            })
        }
    }

    pub fn calibrate(&mut self) -> Calibration {
        self.accelerometer.enable();
        
        Calibration {
            accelerometer: self.accelerometer.calibrate()
        }
    }
}
