#[derive(Copy, Clone)]
pub enum IIRFilterCoefficient {
    Off = 0,
    Two = 2,
    Four = 4,
    Eight = 8,
    Sixteen = 16,
}

impl IIRFilterCoefficient {
    pub fn bits(&self) -> u8 {
        *self as u8
    }
}
