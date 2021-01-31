use crate::iir_filter_coefficient::IIRFilterCoefficient;
use crate::measurement_mode::MeasurementMode;
use crate::measurement_standby_time_millis::MeasurementStandbyTimeMillis;
use crate::pressure_over_sampling::PressureOverSampling;
use crate::temperature_over_sampling::TemperatureOverSampling;

pub struct Config {
    pub mode: MeasurementMode,
    pub pressure_oversampling: PressureOverSampling,
    pub temperature_oversampling: TemperatureOverSampling,
    pub iir_filter: IIRFilterCoefficient,
}

impl Config {
    pub fn handheld_device_lowpower() -> Config {
        Config {
            mode: MeasurementMode::Normal(MeasurementStandbyTimeMillis::SixtyTwoPointFive),
            pressure_oversampling: PressureOverSampling::Sixteen,
            temperature_oversampling: TemperatureOverSampling::Two,
            iir_filter: IIRFilterCoefficient::Four,
        }
    }

    pub fn handheld_device_dynamic() -> Config {
        Config {
            mode: MeasurementMode::Normal(MeasurementStandbyTimeMillis::ZeroPointFive),
            pressure_oversampling: PressureOverSampling::Four,
            temperature_oversampling: TemperatureOverSampling::One,
            iir_filter: IIRFilterCoefficient::Sixteen,
        }
    }

    pub fn weather_monitoring() -> Config {
        Config {
            mode: MeasurementMode::Forced,
            pressure_oversampling: PressureOverSampling::One,
            temperature_oversampling: TemperatureOverSampling::One,
            iir_filter: IIRFilterCoefficient::Off,
        }
    }

    pub fn elevator_floor_monitoring() -> Config {
        Config {
            mode: MeasurementMode::Normal(MeasurementStandbyTimeMillis::OneTwentyFive),
            pressure_oversampling: PressureOverSampling::Four,
            temperature_oversampling: TemperatureOverSampling::One,
            iir_filter: IIRFilterCoefficient::Four,
        }
    }

    pub fn drop_detection() -> Config {
        Config {
            mode: MeasurementMode::Normal(MeasurementStandbyTimeMillis::ZeroPointFive),
            pressure_oversampling: PressureOverSampling::Two,
            temperature_oversampling: TemperatureOverSampling::One,
            iir_filter: IIRFilterCoefficient::Sixteen,
        }
    }

    pub fn indoor_navigation() -> Config {
        Config {
            mode: MeasurementMode::Normal(MeasurementStandbyTimeMillis::ZeroPointFive),
            pressure_oversampling: PressureOverSampling::Sixteen,
            temperature_oversampling: TemperatureOverSampling::Two,
            iir_filter: IIRFilterCoefficient::Sixteen,
        }
    }

    pub fn config_byte(&self) -> u8 {
        let sb_bytes = match self.mode {
            MeasurementMode::Normal(sb) => sb.bits(),
            _ => 0x00000000,
        };
        let filter_bytes = self.iir_filter.bits();

        sb_bytes | filter_bytes
    }

    pub fn ctrl_meas_byte(&self) -> u8 {
        let sample_rate_temp_bytes = self.temperature_oversampling.bits();
        let sample_rate_press_bytes = self.pressure_oversampling.bits();
        let mode_bytes = self.mode.bits();

        sample_rate_press_bytes | sample_rate_temp_bytes | mode_bytes
    }
}
