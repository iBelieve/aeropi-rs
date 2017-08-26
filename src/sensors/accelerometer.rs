use std::{thread, time};
use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use config::AccelerometerCalibration;
use math::{Vec3, twos_comp_combine};


const CALIBRATION_ITERATIONS: u8 = 50;
const ACCELERATION_SCALE_FACTOR: f64 = 0.001;

const LSM_ADDRESS: u16 = 0x1d; // Device I2C slave address
const LSM_WHOAMI_ADDRESS: u8 = 0x0F;
const LSM_WHOAMI_CONTENTS: u8 = 0b1001001; // Device self-id

// LSM_WHOAMI_ADDRESS = 0x0F
// LSM_WHOAMI_CONTENTS = 0b1001001  // Device self-id

// const CTRL_0: u8 = 0x1F;  // General settings
const CTRL_1: u8 = 0x20; // Turns on accelerometer and configures data rate
const CTRL_2: u8 = 0x21; // Self test accelerometer, anti-aliasing accel filter
// const CTRL_3: u8 = 0x22;  // Interrupts
// const CTRL_4: u8 = 0x23;  // Interrupts
const CTRL_5: u8 = 0x24; // Turns on temperature sensor
// const CTRL_6: u8 = 0x25;  // Magnetic resolution selection, data rate config
// const CTRL_7: u8 = 0x26;  // Turns on magnetometer and adjusts mode

const ACC_X_LSB: u8 = 0x28; // x
const ACC_X_MSB: u8 = 0x29;
const ACC_Y_LSB: u8 = 0x2A; // y
const ACC_Y_MSB: u8 = 0x2B;
const ACC_Z_LSB: u8 = 0x2C; // z
const ACC_Z_MSB: u8 = 0x2D;

pub struct Accelerometer {
    i2c: LinuxI2CDevice,
    offsets: Vec3,
}

impl Accelerometer {
    pub fn new() -> Accelerometer {
        Accelerometer {
            i2c: LinuxI2CDevice::new("/dev/i2c-1", LSM_ADDRESS)
                .expect("Unable to connect to the i2c bus"),
            offsets: Vec3::zero(),
        }
    }

    fn enable(&mut self) -> Result<(), LinuxI2CError> {
        self.i2c.smbus_write_byte_data(CTRL_1, 0b10010111)?; // enable accelerometer, 800 hz sampling
        self.i2c.smbus_write_byte_data(CTRL_2, 0x00)?; // set +/- 2g full scale
        self.i2c.smbus_write_byte_data(CTRL_5, 0b01100100)?; // high resolution mode, thermometer off, 6.25hz ODR
        // self.i2c.smbus_write_byte_data(CTRL_6, 0b00100000)  // set +/- 4 gauss full scale
        // self.i2c.smbus_write_byte_data(CTRL_7, 0x00)  // get magnetometer out of low power mode

        Ok(())
    }

    pub fn init(&mut self) {
        let whoami_check = self.i2c.smbus_read_byte_data(LSM_WHOAMI_ADDRESS)
            .expect("Unable to check LSM303D whoami");
        if whoami_check != LSM_WHOAMI_CONTENTS {
            panic!("No LSM303D detected on i2c bus");
        }

        self.enable().expect("Unable to initialize LSM303D");
    }

    pub fn calibrate(&mut self) -> AccelerometerCalibration {
        println!("Calibrating accelerometer...");
        let mut offsets = Vec3::zero();
        let calibration_interval = time::Duration::from_millis(20);

        for _ in 0..CALIBRATION_ITERATIONS {
            offsets += self.read_raw().expect("Unable to read acceleration");
            thread::sleep(calibration_interval);
        }

        self.offsets = offsets / (CALIBRATION_ITERATIONS as f64);

        println!("Calibrated accelerometer, offsets are {}", self.offsets);

        AccelerometerCalibration {
            offsets: self.offsets
        }
    }

    pub fn set_calibration(&mut self, calibration: &AccelerometerCalibration) {
        self.offsets = calibration.offsets;
    }

    pub fn read(&mut self) -> Result<Vec3, LinuxI2CError> {
        Ok((self.read_raw()? - self.offsets) * ACCELERATION_SCALE_FACTOR)
    }

    fn read_raw(&mut self) -> Result<Vec3, LinuxI2CError> {
        Ok(Vec3 {
            x: twos_comp_combine(self.i2c.smbus_read_byte_data(ACC_X_MSB)?,
                                 self.i2c.smbus_read_byte_data(ACC_X_LSB)?) as f64,
            y: twos_comp_combine(self.i2c.smbus_read_byte_data(ACC_Y_MSB)?,
                                 self.i2c.smbus_read_byte_data(ACC_Y_LSB)?) as f64,
            z: twos_comp_combine(self.i2c.smbus_read_byte_data(ACC_Z_MSB)?,
                                 self.i2c.smbus_read_byte_data(ACC_Z_LSB)?) as f64,
        })
    }
}
