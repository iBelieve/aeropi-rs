use led::STATUS_LED;
use sensors::HCSR04;
use std::thread::sleep;
use std::time::Duration;

pub fn run() {
    println!("Executing self test...");
    STATUS_LED.on();
    test_distance_sensor();
    STATUS_LED.off();
    println!("Self test complete!");
}

pub fn test_distance_sensor() {
    println!("Testing distance sensor...");
    let sensor = HCSR04::new();
    sensor.init();

    for _ in 1..10 {
        let distance = sensor.read_raw().expect("Unable to read distance");
        if let Some(distance) = distance {
            println!("Distance: {} cm", distance);
        } else {
            println!("Nothing detected");
        }
        sleep(Duration::from_secs(1));
    }
}
