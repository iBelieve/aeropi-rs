extern crate ctrlc;
extern crate eventual;
extern crate floating_duration;
extern crate i2cdev;
#[macro_use]
extern crate lazy_static;
extern crate nalgebra as na;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate sysfs_gpio;
extern crate ws;

mod led;
mod math;
mod self_test;
mod sensors;
mod socket;
mod status_monitor;
mod utils;

use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let cmd = args.next().unwrap_or(String::from("monitor"));

    println!("AeroPi starting...");

    socket::listen();

    match cmd.as_str() {
        "monitor" => {
            status_monitor::run();
        }
        "test" => {
            self_test::run();
        }
        _ => panic!("Unrecognized command: {}", cmd),
    }

    println!("AeroPi done.");
}
