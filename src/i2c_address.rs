/// BMP280 I2CAddress is configured by pulling the SDO pin high or low.
#[derive(Copy, Clone)]
pub enum I2CAddress {
    SdoGrounded = 0x76,
    SdoPulledUp = 0x77,
}

impl I2CAddress {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}
