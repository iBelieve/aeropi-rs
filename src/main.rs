#[macro_use]
extern crate lazy_static;
extern crate sysfs_gpio;

mod led;
mod self_test;

fn main() {
    println!("Hello, world!");

    self_test::run();
}
