use sysfs_gpio::{Pin, Result};
use std::thread::sleep;
use std::time::Duration;
use super::utils::time_pulse;

const SPEED_OF_SOUND: f64 = 34300.0; // cm/s
const MAX_DISTANCE: f64 = 400.0;
const MAX_WAIT_PERIOD: f64 = 2.0 * MAX_DISTANCE / SPEED_OF_SOUND;

pub struct HCSR04 {
    trigger: Pin,
    echo: Pin,
}

/// HC SR04 ultrasonic distance sensor. See datasheet at http://www.micropik.com/PDF/HCSR04.pdf
impl HCSR04 {
    pub fn new() -> Self {
        let trigger = Pin::from_path("/var/run/gpio/hcsr04_trigger")
            .expect("Unable to find exported hcsr04_trigger GPIO pin");
        let echo = Pin::from_path("/var/run/gpio/hcsr04_echo")
            .expect("Unable to find exported hcsr04_echo GPIO pin");
        HCSR04 { trigger, echo }
    }

    /// Returns distance to object in centimeters.
    pub fn read_raw(&self) -> Result<Option<f64>> {
        self.trigger.set_value(0)?;
        sleep(Duration::new(0, 2_000)); // 2 us (microseconds)
        self.trigger.set_value(1)?;
        sleep(Duration::new(0, 10_000)); // 10 us (microseconds)
        self.trigger.set_value(0)?;

        let duration = match time_pulse(self.echo, 1, MAX_WAIT_PERIOD)? {
            Some(duration) => duration,
            None => return Ok(None),
        };
        let duration_one_way = duration / 2.0;

        let distance = SPEED_OF_SOUND * duration_one_way;

        if distance <= MAX_DISTANCE {
            Ok(Some(distance))
        } else {
            Ok(None)
        }
    }
}
