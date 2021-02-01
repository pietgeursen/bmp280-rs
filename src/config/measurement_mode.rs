use crate::measurement_standby_time_millis::MeasurementStandbyTimeMillis;

#[derive(Copy, Clone)]
pub enum MeasurementMode {
    Sleep,
    Forced,
    Normal(MeasurementStandbyTimeMillis),
}

impl MeasurementMode {
    pub fn bits(&self) -> u8 {
        match self {
            Self::Forced => 0b01,
            Self::Normal(_) => 0b11,
            Self::Sleep => 0b00,
        }
    }
}
