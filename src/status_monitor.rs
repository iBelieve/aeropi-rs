use led::STATUS_LED;
use sensors::{HCSR04, LSM303D};
use socket;
use utils::runloop;

pub fn run() {
    println!("Running self test...");
    STATUS_LED.on();

    let distance_sensor = HCSR04::new();
    let mut accelerometer = LSM303D::new();

    accelerometer.init();

    runloop(1000, || {
        let distance = distance_sensor.read_raw().expect("Unable to read distance");
        let accel = accelerometer
            .read_raw()
            .expect("Unable to read acceleration");

        socket::send_distance(distance).expect("Unable to send distance to socket");
        socket::send_acceleration(accel).expect("Unable to send acceleration to socket");
    });

    STATUS_LED.off();
    println!("Self test complete!");
}
