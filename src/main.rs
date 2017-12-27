extern crate floating_duration;
#[macro_use]
extern crate lazy_static;
extern crate sysfs_gpio;

mod led;
mod self_test;
mod sensors;

fn main() {
    println!("Welcome to AeroPi!");

    self_test::run();
}
