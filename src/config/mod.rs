pub mod iir_filter_coefficient;
pub mod measurement_mode;
pub mod measurement_standby_time_millis;
pub mod pressure_oversampling;
pub mod temperature_oversampling;

pub use iir_filter_coefficient::IIRFilterCoefficient;
pub use measurement_mode::MeasurementMode;
pub use measurement_standby_time_millis::MeasurementStandbyTimeMillis;
pub use pressure_oversampling::PressureOversampling;
pub use temperature_oversampling::TemperatureOversampling;

pub struct Config {
    pub measurement_standby_time_millis: Option<MeasurementStandbyTimeMillis>,
    pub pressure_oversampling: PressureOversampling,
    pub temperature_oversampling: TemperatureOversampling,
    pub iir_filter: IIRFilterCoefficient,
}

impl Config {
    /// Use in NormalMode
    pub fn handheld_device_lowpower() -> Config {
        Config {
            measurement_standby_time_millis: Some(MeasurementStandbyTimeMillis::SixtyTwoPointFive),
            pressure_oversampling: PressureOversampling::Sixteen,
            temperature_oversampling: TemperatureOversampling::Two,
            iir_filter: IIRFilterCoefficient::Four,
        }
    }

    /// Use in NormalMode
    pub fn handheld_device_dynamic() -> Config {
        Config {
            measurement_standby_time_millis: Some(MeasurementStandbyTimeMillis::ZeroPointFive),
            pressure_oversampling: PressureOversampling::Four,
            temperature_oversampling: TemperatureOversampling::One,
            iir_filter: IIRFilterCoefficient::Sixteen,
        }
    }

    /// Use in sleep mode with the [trigger_measurement] methods.
    pub fn weather_monitoring() -> Config {
        Config {
            measurement_standby_time_millis: None,
            pressure_oversampling: PressureOversampling::One,
            temperature_oversampling: TemperatureOversampling::One,
            iir_filter: IIRFilterCoefficient::Off,
        }
    }

    /// Use in NormalMode
    pub fn elevator_floor_monitoring() -> Config {
        Config {
            measurement_standby_time_millis: Some(MeasurementStandbyTimeMillis::OneTwentyFive),
            pressure_oversampling: PressureOversampling::Four,
            temperature_oversampling: TemperatureOversampling::One,
            iir_filter: IIRFilterCoefficient::Four,
        }
    }

    /// Use in NormalMode
    pub fn drop_detection() -> Config {
        Config {
            measurement_standby_time_millis: Some(MeasurementStandbyTimeMillis::ZeroPointFive),
            pressure_oversampling: PressureOversampling::Two,
            temperature_oversampling: TemperatureOversampling::One,
            iir_filter: IIRFilterCoefficient::Sixteen,
        }
    }

    /// Use in NormalMode
    pub fn indoor_navigation() -> Config {
        Config {
            measurement_standby_time_millis: Some(MeasurementStandbyTimeMillis::ZeroPointFive),
            pressure_oversampling: PressureOversampling::Sixteen,
            temperature_oversampling: TemperatureOversampling::Two,
            iir_filter: IIRFilterCoefficient::Sixteen,
        }
    }

    pub(crate) fn config_byte(&self) -> u8 {
        let sb_bytes = match self.measurement_standby_time_millis {
            Some(sb) => sb.bits(),
            _ => 0x00000000,
        };
        let filter_bytes = self.iir_filter.bits();

        sb_bytes | filter_bytes
    }

    pub(crate) fn ctrl_meas_byte(&self, measurement_mode: MeasurementMode) -> u8 {
        let sample_rate_temp_bytes = self.temperature_oversampling.bits();
        let sample_rate_press_bytes = self.pressure_oversampling.bits();
        let mode_bytes = measurement_mode.bits();

        sample_rate_press_bytes | sample_rate_temp_bytes | mode_bytes
    }
}
