use snafu::Snafu;

#[derive(Debug, Snafu)]
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
