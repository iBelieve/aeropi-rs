#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod config;
mod controller;
mod i2c;
mod math;
mod motors;
mod sensors;

use std::env;
use controller::FlightController;

fn main() {
    let mut args = env::args().skip(1);
    let cmd = args.next().unwrap_or(String::from("fly"));

    println!("AeroPi starting...");
    let mut controller = FlightController::new();

    match cmd.as_str() {
        "fly" => {
            controller.init();
        },
        "calibrate" => {
            controller.calibrate();
        },
        _ => panic!("Unrecognized command: {}", cmd)
    }

    println!("AeroPi done.");
}
