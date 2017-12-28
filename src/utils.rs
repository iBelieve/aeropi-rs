use sysfs_gpio::{Error, Pin, Result};
use std::time::SystemTime;
use floating_duration::TimeAsFloat;
use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use eventual::Timer;

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

pub fn runloop<F: FnMut()>(interval_ms: u32, mut tick: F) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let timer = Timer::new();
    let ticks = timer.interval_ms(interval_ms).iter();
    for _ in ticks {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        tick();
    }
}
