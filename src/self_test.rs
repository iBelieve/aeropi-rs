use led::STATUS_LED;
use sensors::{HCSR04, LSM303D};

pub fn run() {
    println!("Executing self test...");
    STATUS_LED.on();
    test_distance_sensor();
    test_accelerometer();
    STATUS_LED.off();
    println!("Self test complete!");
}

pub fn test_distance_sensor() {
    println!("Testing distance sensor...");
    let sensor = HCSR04::new();

    if let Some(distance) = sensor.read_raw().expect("Unable to read distance") {
        println!("Distance: {} cm", distance);
    } else {
        println!("Nothing detected");
    }
}

pub fn test_accelerometer() {
    println!("Testing accelerometer...");
    let mut sensor = LSM303D::new();
    sensor.init();
    let accel = sensor.read_raw().expect("Unable to read acceleration");
    println!("Acceleration: {}", accel);
}
