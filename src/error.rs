use snafu::Snafu;
use defmt::Format;

#[derive(Debug, Snafu, Format)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    IncorrectDeviceId,
    ReadingDeviceAddressOverI2C,
    WriteConfig,
    WriteControlMeas,
    WriteRegister,
    ReadRegister,
    NormalModeNeedsMeasStandbyTime,
}
