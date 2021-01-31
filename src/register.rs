pub enum Register {
    TempXLSB = 0xFC,
    TempLSB = 0xFB,
    TempMSB = 0xFA,
    PressXLSB = 0xF9,
    PressLSB = 0xF8,
    PressMSB = 0xF7,
    Config = 0xF5,
    CtrlMeas = 0xF4,
    Status = 0xF3,
    Reset = 0xE0,
    Id = 0xD0,

    // Calibration values
    CalT1Byte0 = 0x88,
    CalT1Byte1,

    CalT2Byte0,
    CalT2Byte1,

    CalT3Byte0,
    CalT3Byte1,

    CalP1Byte0,
    CalP1Byte1,

    CalP2Byte0,
    CalP2Byte1,

    CalP3Byte0,
    CalP3Byte1,

    CalP4Byte0,
    CalP4Byte1,

    CalP5Byte0,
    CalP5Byte1,

    CalP6Byte0,
    CalP6Byte1,

    CalP7Byte0,
    CalP7Byte1,

    CalP8Byte0,
    CalP8Byte1,

    CalP9Byte0,
    CalP9Byte1,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
    pub fn is_read_only(&self) -> bool {
        match self {
            Register::Config | Register::Reset => false,
            _ => true,
        }
    }
}
