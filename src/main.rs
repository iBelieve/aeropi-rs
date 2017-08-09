mod controller;
mod i2c;
mod math;
mod motors;
mod sensors;

use controller::FlightController;

fn main() {
    println!("AeroPi starting...");
    let mut controller = FlightController::new();

    controller.init();

    println!("AeroPi done.");
}
