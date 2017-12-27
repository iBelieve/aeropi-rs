use sysfs_gpio::{Error, Pin, Result};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use floating_duration::TimeAsFloat;

const SPEED_OF_SOUND: f64 = 34300.0; // cm/s

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

    // TODO: Think about how we can do this without needing to block other operations
    pub fn init(&self) {
        self.trigger
            .set_value(0)
            .expect("Unable to communicate with HC SR04");
        sleep(Duration::from_secs(2));
    }

    /// Returns distance to object in centimeters.
    pub fn read_raw(&self) -> Result<Option<f64>> {
        self.trigger.set_value(1)?;
        sleep(Duration::new(0, 10_000)); // 10 us (microseconds)
        self.trigger.set_value(0)?;

        while self.echo.get_value()? == 0 {}
        let start_time = SystemTime::now();

        // TODO: Return None after value is high for longer than equivalent to 400 cm
        while self.echo.get_value()? == 1 {}
        let end_time = SystemTime::now();

        let duration = end_time
            .duration_since(start_time)
            .map_err(|_| {
                Error::Unexpected(String::from("Unable to get duration of echo"))
            })?
            .as_fractional_secs();
        let duration_one_way = duration / 2.0;

        let distance = SPEED_OF_SOUND * duration_one_way;

        if distance <= 400.0 {
            Ok(Some(distance))
        } else {
            Ok(None)
        }
    }
}
