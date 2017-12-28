use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use math::{twos_comp_combine, Vec3};

const LSM_ADDRESS: u16 = 0b11101;
const LSM_WHOAMI: u8 = 0x0F;
const LSM_ID: u8 = 0b01001001;

const LSM_CTRL_1: u8 = 0x20;
// const LSM_CTRL_2: u8 = 0x21;
// const LSM_CTRL_3: u8 = 0x22;
// const LSM_CTRL_4: u8 = 0x23;
const LSM_CTRL_5: u8 = 0x24;

const LSM_OUT_X_L_A: u8 = 0x28;
const LSM_OUT_X_H_A: u8 = 0x29;
const LSM_OUT_Y_L_A: u8 = 0x2A;
const LSM_OUT_Y_H_A: u8 = 0x2B;
const LSM_OUT_Z_L_A: u8 = 0x2C;
const LSM_OUT_Z_H_A: u8 = 0x2D;

#[derive(Debug)]
pub enum LSMError {
    I2C(LinuxI2CError),
}

impl From<LinuxI2CError> for LSMError {
    fn from(error: LinuxI2CError) -> Self {
        LSMError::I2C(error)
    }
}

pub struct LSM303D {
    i2c: LinuxI2CDevice,
}

impl LSM303D {
    pub fn new() -> Self {
        let i2c = LinuxI2CDevice::new("/dev/i2c-1", LSM_ADDRESS)
            .expect("Unable to connect to I2C bus");

        LSM303D { i2c }
    }

    pub fn init(&mut self) {
        let id = self.i2c
            .smbus_read_byte_data(LSM_WHOAMI)
            .expect("Unable to communicate on I2C bus");

        if id != LSM_ID {
            panic!("No LSM30D detected on I2C bus!");
        }

        self.enable().expect("Unable to initialize LSM303D");
    }

    fn enable(&mut self) -> Result<(), LSMError> {
        self.i2c.smbus_write_byte_data(LSM_CTRL_1, 0b10010111)?; // enable accelerometer, 800 hz sampling
        self.i2c.smbus_write_byte_data(LSM_CTRL_5, 0b01100100)?; // high resolution mode, thermometer off, 6.25hz ODR

        Ok(())
    }

    pub fn read_raw(&mut self) -> Result<Vec3, LSMError> {
        let accel_x = twos_comp_combine(
            self.i2c.smbus_read_byte_data(LSM_OUT_X_H_A)?,
            self.i2c.smbus_read_byte_data(LSM_OUT_X_L_A)?,
        ) as f64;
        let accel_y = twos_comp_combine(
            self.i2c.smbus_read_byte_data(LSM_OUT_Y_H_A)?,
            self.i2c.smbus_read_byte_data(LSM_OUT_Y_L_A)?,
        ) as f64;
        let accel_z = twos_comp_combine(
            self.i2c.smbus_read_byte_data(LSM_OUT_Z_H_A)?,
            self.i2c.smbus_read_byte_data(LSM_OUT_Z_L_A)?,
        ) as f64;

        Ok(Vec3::new(accel_x, accel_y, accel_z))
    }
}
