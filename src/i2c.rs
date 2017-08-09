pub struct I2C {
    address: u8
}

impl I2C {
    pub fn new(address: u8) -> Self {
        I2C { address }
    }

    pub fn whoami(&mut self, id: u8, error: &str) {
        // TODO: Implement this
    }

    pub fn write8(&mut self, register: u8, value: u8) {

    }

    pub fn read8(&mut self, register: u8) -> u8 {
        0
    }
}
