use sysfs_gpio::{Error, Pin, Result};
use std::time::SystemTime;
use floating_duration::TimeAsFloat;

fn seconds_since(time: SystemTime) -> Result<f64> {
    let seconds = SystemTime::now()
        .duration_since(time)
        .map_err(|_| {
            Error::Unexpected(String::from("Unable to get duration of echo"))
        })?
        .as_fractional_secs();
    Ok(seconds)
}

pub fn time_pulse(pin: Pin, value: u8, timeout: f64) -> Result<Option<f64>> {
    while pin.get_value()? != value {}
    let start_time = SystemTime::now();

    while pin.get_value()? == value {
        if seconds_since(start_time)? > timeout {
            return Ok(None);
        }
    }

    let duration = seconds_since(start_time)?;

    Ok(Some(duration))
}
