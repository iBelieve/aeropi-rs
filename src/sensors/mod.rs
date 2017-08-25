mod accelerometer;

use config::Calibration;
use self::accelerometer::Accelerometer;

pub struct SensorReadings {
    pub pitch_rate: f64,
    pub roll_rate: f64,
    pub yaw_rate: f64,
    pub pitch: f64,
    pub roll: f64,
    pub yaw: f64,
    pub aligned_vx: f64,
    pub aligned_vy: f64,
    pub quad_vz: f64
}

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
        self.accelerometer.init();

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
        self.accelerometer.init();

        Calibration {
            accelerometer: self.accelerometer.calibrate()
        }
    }

    pub fn read(&mut self) -> SensorReadings {
        // TODO: Replace with real readings from the sensors
        SensorReadings {
            pitch_rate: 0.0,
            roll_rate: 0.0,
            yaw_rate: 0.0,
            pitch: 0.0,
            roll: 0.0,
            yaw: 0.0,
            aligned_vx: 0.0,
            aligned_vy: 0.0,
            quad_vz: 0.0
        }
    }
}
