use sysfs_gpio::Pin;

lazy_static! {
    pub static ref STATUS_LED: LED = LED::new("status_led");
}

pub struct LED {
    pin: Pin,
}

impl LED {
    pub fn new(name: &str) -> Self {
        let pin =
            Pin::from_path(format!("/var/run/gpio/{}", name)).expect("Unable to connect to LED");

        LED { pin }
    }

    pub fn on(&self) {
        self.pin.set_value(1).expect("Unable to turn LED on.");
    }

    pub fn off(&self) {
        self.pin.set_value(0).expect("Unable to turn LED off.");
    }
}
