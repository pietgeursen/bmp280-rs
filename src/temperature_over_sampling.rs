#[derive(Copy, Clone)]
pub enum TemperatureOverSampling {
    MeasurementSkipped = 0b00000000,
    One = 0b00100000,
    Two = 0b01000000,
    Four = 0b01100000,
    Eight = 0b10000000,
    Sixteen = 0b11100000,
}

impl TemperatureOverSampling {
    pub fn bits(&self) -> u8 {
        *self as u8
    }
}
