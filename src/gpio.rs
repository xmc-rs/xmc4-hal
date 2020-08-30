
#![allow(dead_code)]

use crate::device::port0::RegisterBlock;
use crate::device::PORT0;

const IOCR_PC_SIZE: u32 = 8;

pub struct Gpio {
    port0: *const RegisterBlock
}

pub enum OutputLevel {
    /// Reset bit
    Low = 0x10000,
    ///Set bit
    High = 1
}

impl From<u32> for OutputLevel {
    fn from(bits: u32) -> Self {
        match bits {
            0x10000 => OutputLevel::Low,
            1 => OutputLevel::High,
            _ => unimplemented!(),
        }
    }
}

impl From<OutputLevel> for u32 {
    fn from(level: OutputLevel) -> Self {
        match level {
            OutputLevel::Low => 0x10000,
            OutputLevel::High => 1,
        }
    }
}

pub enum HardwareControl {
    /// Software control only.
    Disabled,
    /// HWI0/HWO0 control path can override the software configuration
    Peripheral1,
    /// HWI1/HWO1 control path can override the software configuration
    Peripheral2
}

impl From<u32> for HardwareControl {
    fn from(bits: u32) -> Self {
        match bits {
            0 => HardwareControl::Disabled,
            1 => HardwareControl::Peripheral1,
            2 => HardwareControl::Peripheral2,
            _ => unimplemented!(),
        }
    }
}

impl From<HardwareControl> for u32 {
    fn from(ctrl: HardwareControl) -> Self {
        match ctrl {
            HardwareControl::Disabled => 0,
            HardwareConrtol::Peripheral1 => 1,
            HardwareControl::Peripheral2 => 2,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}