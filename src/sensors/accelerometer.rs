use std::str::FromStr;
use std::{thread, time};
use std::fs::{self, File};
use std::io::prelude::*;
use i2c::I2C;
use config::AccelerometerCalibration;
use math::{twos_comp_combine, Vec3};

const CALIBRATION_ITERATIONS: u8 = 50;
const ACCELERATION_SCALE_FACTOR: f32 = 0.001;
const CALIBRATION_DATA_FILENAME: &str = "data/accelerometer_calibration.dat";

const LSM_ADDRESS: u8 = 0x1d; // Device I2C slave address
const LSM_WHOAMI: u8 = 0b1001001; // Device self-id

// LSM_WHOAMI_ADDRESS = 0x0F
// LSM_WHOAMI_CONTENTS = 0b1001001  // Device self-id

const CTRL_0: u8 = 0x1F;  // General settings
const CTRL_1: u8 = 0x20;  // Turns on accelerometer and configures data rate
const CTRL_2: u8 = 0x21;  // Self test accelerometer, anti-aliasing accel filter
const CTRL_3: u8 = 0x22;  // Interrupts
const CTRL_4: u8 = 0x23;  // Interrupts
const CTRL_5: u8 = 0x24;  // Turns on temperature sensor
const CTRL_6: u8 = 0x25;  // Magnetic resolution selection, data rate config
const CTRL_7: u8 = 0x26;  // Turns on magnetometer and adjusts mode

const ACC_X_LSB: u8 = 0x28;  // x
const ACC_X_MSB: u8 = 0x29;
const ACC_Y_LSB: u8 = 0x2A;  // y
const ACC_Y_MSB: u8 = 0x2B;
const ACC_Z_LSB: u8 = 0x2C;  // z
const ACC_Z_MSB: u8 = 0x2D;

pub struct Accelerometer {
    i2c: I2C,
    offsets: Vec3
}

impl Accelerometer {
    pub fn new() -> Self {
        Accelerometer {
            i2c: I2C::new(LSM_ADDRESS),
            offsets: Vec3::zero()
        }
    }

    pub fn enable(&mut self) {
        self.i2c.whoami(LSM_WHOAMI, "No LSM303D detected on i2c bus.");
        self.i2c.write8(CTRL_1, 0b10010111);  // enable accelerometer, 800 hz sampling
        self.i2c.write8(CTRL_2, 0x00);  // set +/- 2g full scale
        self.i2c.write8(CTRL_5, 0b01100100);  // high resolution mode, thermometer off, 6.25hz ODR
        // self.i2c.write8(CTRL_6, 0b00100000)  // set +/- 4 gauss full scale
        // self.i2c.write8(CTRL_7, 0x00)  // get magnetometer out of low power mode
    }

    pub fn calibrate(&mut self) -> AccelerometerCalibration {
        println!("Calibrating accelerometer...");
        let mut offsets = Vec3::zero();
        let calibration_interval = time::Duration::from_millis(20);

        for _ in 0..CALIBRATION_ITERATIONS {
            offsets += self.read_raw();
            thread::sleep(calibration_interval);
        }

        self.offsets = offsets / (CALIBRATION_ITERATIONS as f32);

        println!("Calibrated accelerometer, offsets are {}", self.offsets);

        AccelerometerCalibration {
            offsets: self.offsets
        }
    }

    pub fn set_calibration(&mut self, calibration: &AccelerometerCalibration) {
        self.offsets = calibration.offsets;
    }

    pub fn read(&mut self) -> Vec3 {
        (self.read_raw() - self.offsets) * ACCELERATION_SCALE_FACTOR
    }

    fn read_raw(&mut self) -> Vec3 {
        Vec3 {
            x: twos_comp_combine(self.i2c.read8(ACC_X_MSB), self.i2c.read8(ACC_X_LSB)) as f32,
            y: twos_comp_combine(self.i2c.read8(ACC_Y_MSB), self.i2c.read8(ACC_Y_LSB)) as f32,
            z: twos_comp_combine(self.i2c.read8(ACC_Z_MSB), self.i2c.read8(ACC_Z_LSB)) as f32
        }
    }
}
