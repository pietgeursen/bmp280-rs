#[derive(Copy, Clone)]
pub enum PressureOverSampling {
    MeasurementSkipped = 0b00000,
    One = 0b00100,
    Two = 0b01000,
    Four = 0b01100,
    Eight = 0b10000,
    Sixteen = 0b11100,
}

impl PressureOverSampling {
    pub fn bits(&self) -> u8 {
        *self as u8
    }
}
