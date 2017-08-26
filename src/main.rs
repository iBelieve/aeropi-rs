extern crate aeropi;

use std::env;
use aeropi::FlightController;

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
