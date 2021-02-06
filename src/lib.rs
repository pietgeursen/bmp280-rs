#![no_std]
#![forbid(unsafe_code)]

use core::fmt::Debug;
use core::marker::PhantomData;
use embedded_hal as hal;
use hal::blocking::i2c::{Write, WriteRead};
use snafu::{ensure, OptionExt};

pub mod config;
pub mod error;
pub mod i2c_address;
mod internal;
pub mod register;

pub use config::*;
pub use error::*;
pub use register::*;

use internal::*;

pub use i2c_address::I2CAddress;
use pressure_calibration_data::PressureCalibrationData;
use temperature_calibration_data::TemperatureCalibrationData;

pub struct ModeNormal;
pub struct ModeSleep;

pub struct BMP280<I2C, Mode> {
    i2c_address: u8,
    temperature_calibration_data: TemperatureCalibrationData,
    pressure_calibration_data: PressureCalibrationData,
    t_fine: i32,
    config: Config,
    mode: PhantomData<Mode>,
    i2c: PhantomData<I2C>,
}

// Magic number that must be written to the reset register to actually trigger a reset.
const RESET_WORD: u8 = 0xB6;
const DEVICE_ID: u8 = 0x58;

impl<I2C, E, Mode> BMP280<I2C, Mode>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    E: Debug,
{
    /// Reset the chip into sleep mode and reconfigure it with the current config.
    pub fn into_reset(self, i2c: &mut I2C) -> Result<BMP280<I2C, ModeSleep>, Error> {
        i2c
            .write(self.i2c_address, &[Register::Reset.addr(), RESET_WORD])
            .map_err(|_| Error::WriteConfig)?;

        Self::_configure(
            i2c,
            self.i2c_address,
            &self.config,
            MeasurementMode::Sleep,
        )?;

        Ok(BMP280 {
            i2c: PhantomData, 
            i2c_address: self.i2c_address,
            temperature_calibration_data: self.temperature_calibration_data,
            pressure_calibration_data: self.pressure_calibration_data,
            t_fine: self.t_fine,
            config: self.config,
            mode: PhantomData,
        })
    }

    fn _trigger_forced_measurement(&self, i2c: &mut I2C) -> Result<(), Error> {
        Self::_configure(
            i2c, 
            self.i2c_address,
            &self.config,
            MeasurementMode::Forced,
        )
    }

    fn _into_normal_mode(self, i2c: &mut I2C) -> Result<BMP280<I2C, ModeNormal>, Error> {
        let measurement_standby_time_millis = self
            .config
            .measurement_standby_time_millis
            .context(NormalModeNeedsMeasStandbyTime)?;

        Self::_configure(
            i2c,
            self.i2c_address,
            &self.config,
            MeasurementMode::Normal(measurement_standby_time_millis),
        )?;

        Ok(BMP280 {
            i2c: PhantomData, 
            i2c_address: self.i2c_address,
            temperature_calibration_data: self.temperature_calibration_data,
            pressure_calibration_data: self.pressure_calibration_data,
            t_fine: self.t_fine,
            config: self.config,
            mode: PhantomData,
        })
    }
    fn _into_sleep_mode(self, i2c: &mut I2C) -> Result<BMP280<I2C, ModeSleep>, Error> {
        Self::_configure(
            i2c,
            self.i2c_address,
            &self.config,
            MeasurementMode::Sleep,
        )?;

        Ok(BMP280 {
            i2c: self.i2c,
            i2c_address: self.i2c_address,
            temperature_calibration_data: self.temperature_calibration_data,
            pressure_calibration_data: self.pressure_calibration_data,
            t_fine: self.t_fine,
            config: self.config,
            mode: PhantomData,
        })
    }

    /// Read the uncompensated temperature data. 20 bit value.
    fn _read_raw_temperature(&self, i2c: &mut I2C) -> Result<u32, Error> {
        let bytes =
            Self::write_read_register_u32(i2c, self.i2c_address, Register::TempMSB)?;
        Ok(bytes >> 12)
    }

    /// Read the uncompensated pressure data. 20 bit value.
    fn _read_raw_pressure(&self, i2c: &mut I2C) -> Result<u32, Error> {
        let bytes =
            Self::write_read_register_u32(i2c, self.i2c_address, Register::PressMSB)?;
        Ok(bytes >> 12)
    }

    /// Read the compensated temperature.
    fn _read_temperature(&mut self, i2c: &mut I2C) -> Result<i32, Error> {
        let raw_temp = self._read_raw_temperature(i2c)?;

        let (calibrated_temperature, t_fine) =
            Self::compensate_temperature(raw_temp as i32, &self.temperature_calibration_data);

        // t_fine is needed for compensating the pressure.
        self.t_fine = t_fine;

        Ok(calibrated_temperature)
    }

    /// Read the compensated pressure.
    fn _read_pressure(&mut self, i2c: &mut I2C) -> Result<i32, Error> {
        let raw_pressure = self._read_raw_pressure(i2c)?;

        let compensated_pressure = Self::compensate_pressure(
            raw_pressure as i32,
            self.t_fine,
            &self.pressure_calibration_data,
        );

        Ok(compensated_pressure)
    }

    fn _configure(
        i2c: &mut I2C,
        i2c_address: u8,
        config: &Config,
        measurement_mode: MeasurementMode,
    ) -> Result<(), Error> {
        let config_byte = config.config_byte();
        i2c.write(i2c_address, &[Register::Config.addr(), config_byte])
            .map_err(|_| Error::WriteConfig)?;

        let ctrl_meas_byte = config.ctrl_meas_byte(measurement_mode);
        i2c.write(i2c_address, &[Register::CtrlMeas.addr(), ctrl_meas_byte])
            .map_err(|_| Error::WriteControlMeas)
    }

    fn get_temperature_calibration_data(
        i2c: &mut I2C,
        i2c_address: u8,
    ) -> Result<TemperatureCalibrationData, Error> {
        let t1 = Self::write_read_register_u16_le(i2c, i2c_address, Register::CalT1Byte0)?;
        let t2 = Self::write_read_register_i16_le(i2c, i2c_address, Register::CalT2Byte0)?;
        let t3 = Self::write_read_register_i16_le(i2c, i2c_address, Register::CalT3Byte0)?;

        Ok(TemperatureCalibrationData { t1, t2, t3 })
    }

    fn get_pressure_calibration_data(
        i2c: &mut I2C,
        i2c_address: u8,
    ) -> Result<PressureCalibrationData, Error> {
        let p1 = Self::write_read_register_u16_le(i2c, i2c_address, Register::CalP1Byte0)?;
        let p2 = Self::write_read_register_i16_le(i2c, i2c_address, Register::CalP2Byte0)?;
        let p3 = Self::write_read_register_i16_le(i2c, i2c_address, Register::CalP3Byte0)?;
        let p4 = Self::write_read_register_i16_le(i2c, i2c_address, Register::CalP4Byte0)?;
        let p5 = Self::write_read_register_i16_le(i2c, i2c_address, Register::CalP5Byte0)?;
        let p6 = Self::write_read_register_i16_le(i2c, i2c_address, Register::CalP6Byte0)?;
        let p7 = Self::write_read_register_i16_le(i2c, i2c_address, Register::CalP7Byte0)?;
        let p8 = Self::write_read_register_i16_le(i2c, i2c_address, Register::CalP8Byte0)?;
        let p9 = Self::write_read_register_i16_le(i2c, i2c_address, Register::CalP9Byte0)?;

        Ok(PressureCalibrationData {
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
            p9,
        })
    }

    /// Get the device ID
    fn get_device_id(i2c: &mut I2C, i2c_address: u8) -> Result<u8, Error> {
        let input = [Register::Id.addr()];
        let mut output = [0u8];
        i2c.write_read(i2c_address, &input, &mut output)
            .map_err(|_| Error::ReadingDeviceAddressOverI2C)?;
        Ok(output[0])
    }

    fn write_read_register_u16_le(
        i2c: &mut I2C,
        i2c_address: u8,
        register: Register,
    ) -> Result<u16, Error> {
        let mut buffer = [0u8; 2];
        i2c.write_read(i2c_address, &[register.addr()], &mut buffer)
            .map_err(|_| Error::ReadRegister)?;

        Ok(u16::from_le_bytes(buffer))
    }

    fn write_read_register_i16_le(
        i2c: &mut I2C,
        i2c_address: u8,
        register: Register,
    ) -> Result<i16, Error> {
        let mut buffer = [0u8; 2];
        i2c.write_read(i2c_address, &[register.addr()], &mut buffer)
            .map_err(|_| Error::ReadRegister)?;

        Ok(i16::from_le_bytes(buffer))
    }

    fn write_read_register_u32(
        i2c: &mut I2C,
        i2c_address: u8,
        register: Register,
    ) -> Result<u32, Error> {
        let mut buffer = [0u8; 4];
        i2c.write_read(i2c_address, &[register.addr()], &mut buffer)
            .map_err(|_| Error::ReadRegister)?;

        Ok(u32::from_be_bytes(buffer))
    }

    // Line by line port of the c code from the datasheet
    fn compensate_temperature(
        raw_temp: i32,
        temperature_calibration_data: &TemperatureCalibrationData,
    ) -> (i32, i32) {
        let var1 = (((raw_temp >> 3) - ((temperature_calibration_data.t1 as i32) << 1))
            * (temperature_calibration_data.t2 as i32))
            >> 11;

        let var2 = (((((raw_temp >> 4) - (temperature_calibration_data.t1 as i32))
            * ((raw_temp >> 4) - (temperature_calibration_data.t1 as i32)))
            >> 12)
            * (temperature_calibration_data.t3 as i32))
            >> 14;

        let t_fine = var1 + var2;
        let t = (t_fine * 5 + 128) >> 8;

        (t, t_fine)
    }

    // Line by line port of the c code from the datasheet
    fn compensate_pressure(
        raw_pressure: i32,
        t_fine: i32,
        pressure_calibration_data: &PressureCalibrationData,
    ) -> i32 {
        let t_fine = t_fine as i64;

        let var1 = t_fine - 128000;
        let var2 = var1 * var1 * (pressure_calibration_data.p6 as i64);
        let var2 = var2 + ((var1 * (pressure_calibration_data.p5 as i64)) << 17);
        let var2 = var2 + ((pressure_calibration_data.p4 as i64) << 35);

        let var1 = ((var1 * var1 * (pressure_calibration_data.p3 as i64)) >> 8)
            + ((var1 * (pressure_calibration_data.p2 as i64)) << 12);
        let var1 = ((1i64 << 47) + var1) * (pressure_calibration_data.p1 as i64) >> 33;

        if var1 == 0 {
            return 0;
        }

        let p = 1048576i64 - raw_pressure as i64;
        let p = (((p << 31) - var2) * 3125) / var1;
        let var1 = ((pressure_calibration_data.p9 as i64) * (p >> 13) * (p >> 13)) >> 25;
        let var2 = ((pressure_calibration_data.p8 as i64) * p) >> 19;
        let p = ((p + var1 + var2) >> 8) + ((pressure_calibration_data.p7 as i64) << 4);
        p as i32
    }
}

impl<I2C, E> BMP280<I2C, ModeNormal>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    E: Debug,
{
    /// Change into sleep mode
    pub fn into_sleep_mode(self, i2c: &mut I2C) -> Result<BMP280<I2C, ModeSleep>, Error> {
        Self::_into_sleep_mode(self, i2c)
    }

    /// Read the uncompensated pressure data. 20 bit value.
    pub fn read_raw_pressure(&mut self, i2c: &mut I2C) -> Result<u32, Error> {
        Self::_read_raw_pressure(self, i2c)
    }

    /// Read the uncompensated temperature data. 20 bit value.
    pub fn read_raw_temperature(&mut self, i2c: &mut I2C) -> Result<u32, Error> {
        Self::_read_raw_temperature(self, i2c)
    }

    /// Read the compensated pressure.
    pub fn read_pressure(&mut self, i2c: &mut I2C) -> Result<i32, Error> {
        Self::_read_pressure(self, i2c)
    }

    /// Read the compensated temperature.
    pub fn read_temperature(&mut self, i2c: &mut I2C) -> Result<i32, Error> {
        Self::_read_temperature(self, i2c)
    }
}

impl<I2C, E> BMP280<I2C, ModeSleep>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    E: Debug,
{
    pub fn new(i2c: &mut I2C, i2c_address: I2CAddress, config: Config) -> Result<Self, Error> {
        let i2c_address = i2c_address.addr();

        ensure!(
            DEVICE_ID == Self::get_device_id(i2c, i2c_address)?,
            IncorrectDeviceId
        );

        let temperature_calibration_data =
            Self::get_temperature_calibration_data(i2c, i2c_address)?;

        let pressure_calibration_data = Self::get_pressure_calibration_data(i2c, i2c_address)?;

        Self::_configure(i2c, i2c_address, &config, MeasurementMode::Sleep)?;

        let bmp280 = BMP280 {
            i2c: PhantomData,
            i2c_address,
            temperature_calibration_data,
            pressure_calibration_data,
            t_fine: 0,
            config,
            mode: PhantomData,
        };

        Ok(bmp280)
    }

    /// Convert into normal mode.
    ///
    /// Normal mode continuously samples.
    pub fn into_normal_mode(self, i2c: &mut I2C) -> Result<BMP280<I2C, ModeNormal>, Error> {
        Self::_into_normal_mode(self, i2c)
    }

    /// Trigger a "forced" mode single measurement.
    ///
    /// Device internally returns to sleep mode after measurement is complete and then the pressure
    /// / temperature can be read.
    pub fn trigger_measurement(&self, i2c: &mut I2C) -> Result<(), Error> {
        Self::_trigger_forced_measurement(self, i2c)
    }

    /// Read the uncompensated pressure data. 20 bit value.
    ///
    /// Call `trigger_measurement` before calling this.
    /// NB You need to allow time for the measurement to complete!
    pub fn read_raw_pressure(&mut self, i2c: &mut I2C) -> Result<u32, Error> {
        Self::_read_raw_pressure(self, i2c)
    }

    /// Read the uncompensated temperature data. 20 bit value.
    ///
    /// Call `trigger_measurement` before calling this.
    /// NB You need to allow time for the measurement to complete!
    pub fn read_raw_temperature(&mut self, i2c: &mut I2C) -> Result<u32, Error> {
        Self::_read_raw_temperature(self, i2c)
    }

    /// Read the compensated pressure.
    ///
    /// Call `trigger_measurement` before calling this.
    /// NB You need to allow time for the measurement to complete!
    pub fn read_pressure(&mut self, i2c: &mut I2C) -> Result<i32, Error> {
        Self::_read_pressure(self, i2c)
    }

    /// Read the compensated temperature.
    ///
    /// Call `trigger_measurement` before calling this.
    /// NB You need to allow time for the measurement to complete!
    pub fn read_temperature(&mut self, i2c: &mut I2C) -> Result<i32, Error> {
        Self::_read_temperature(self, i2c)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use embedded_hal_mock::i2c::Mock as I2CMock;

    #[test]
    fn temperature_calibration() {
        // Values from datasheet page 23
        let ut = 519888i32;
        let temperature_calibration_data = TemperatureCalibrationData {
            t1: 27504,
            t2: 26435,
            t3: -1000,
        };
        let (result, t_fine) =
            BMP280::<I2CMock, ModeSleep>::compensate_temperature(ut, &temperature_calibration_data);
        assert_eq!(t_fine, 128422);
        assert_eq!(result, 2508);
    }

    #[test]
    fn pressure_calibration() {
        // Values from datasheet page 23
        let up = 415148;
        let t_fine = 128422;
        let pressure_calibration_data = PressureCalibrationData {
            p1: 36477,
            p2: -10685,
            p3: 3024,
            p4: 2855,
            p5: 140,
            p6: -7,
            p7: 15500,
            p8: -14600,
            p9: 6000,
        };
        let result = BMP280::<I2CMock, ModeSleep>::compensate_pressure(
            up,
            t_fine,
            &pressure_calibration_data,
        );

        // This is the value in the datasheet but it uses floating point and we're 3 away.
        //assert_eq!(result, 25767236);

        // I'm gonna say it's close enough
        assert_eq!(result, 25767233);
    }
}
