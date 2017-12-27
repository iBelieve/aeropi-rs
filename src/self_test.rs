use led::STATUS_LED;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
    STATUS_LED.on();
    sleep(Duration::from_secs(2));
    STATUS_LED.off();
}
