#[derive(Copy, Clone)]
pub enum MeasurementStandbyTimeMillis {
    ZeroPointFive = 0b00000000,
    SixtyTwoPointFive = 0b00100000,
    OneTwentyFive = 0b01000000,
    TwoFifty = 0b01100000,
    FiveHundred = 0b10000000,
    OneThousand = 0b10100000,
    TwoThousand = 0b11000000,
    FourThousand = 0b11100000,
}

impl MeasurementStandbyTimeMillis {
    pub fn bits(&self) -> u8 {
        *self as u8
    }
}
