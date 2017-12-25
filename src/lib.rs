extern crate i2cdev;
extern crate nalgebra as na;
extern crate pid_control;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod config;
mod controller;
mod math;
mod motors;
mod sensors;

pub use controller::FlightController;
