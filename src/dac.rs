//! Digital to analog converter. Two channels, each with single-value, data,
//! pattern, noise, and ramp modes.

use crate::pac::DAC;
#[cfg(not(feature = "xmc4500"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Mode {
    Idle = 0,
    Single = 1,
    Data = 2,
    Pattern = 3,
    Noise = 4,
    Ramp = 5,
}

pub trait DacExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Dac;
}

impl Sealed for DAC {}

impl DacExt for DAC {
    fn constrain(self, scu: &mut Scu) -> Dac {
        let dac = Dac { regs: self };
        dac.enable(scu);
        dac
    }
}

pub struct Dac {
    regs: DAC,
}

impl Dac {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(not(feature = "xmc4500"))]
        scu.ungate_peripheral_clock(PeripheralClock::Dac);
        scu.deassert_peripheral_reset(PeripheralReset::Dac);
    }

    pub fn disable(&self, scu: &mut Scu) {
        scu.assert_peripheral_reset(PeripheralReset::Dac);
        #[cfg(not(feature = "xmc4500"))]
        scu.gate_peripheral_clock(PeripheralClock::Dac);
    }

    pub fn set_mode(&self, channel: u8, mode: Mode) -> Result<(), u8> {
        match channel {
            0 => {
                self.regs
                    .dac0cfg0()
                    .modify(|_, w| unsafe { w.mode().bits(mode as u8) });
                Ok(())
            }
            1 => {
                self.regs
                    .dac1cfg0()
                    .modify(|_, w| unsafe { w.mode().bits(mode as u8) });
                Ok(())
            }
            _ => Err(channel),
        }
    }

    /// Power the analog section of one channel.
    pub fn enable_channel(&self, channel: u8) -> Result<(), u8> {
        match channel {
            0 => {
                self.regs.dac0cfg1().modify(|_, w| w.anaen().set_bit());
                Ok(())
            }
            1 => {
                self.regs.dac1cfg1().modify(|_, w| w.anaen().set_bit());
                Ok(())
            }
            _ => Err(channel),
        }
    }

    pub fn disable_channel(&self, channel: u8) -> Result<(), u8> {
        match channel {
            0 => {
                self.regs.dac0cfg1().modify(|_, w| w.anaen().clear_bit());
                Ok(())
            }
            1 => {
                self.regs.dac1cfg1().modify(|_, w| w.anaen().clear_bit());
                Ok(())
            }
            _ => Err(channel),
        }
    }

    /// Single-value or data-mode sample. The DAC is 12-bit.
    pub fn write(&self, channel: u8, sample: u16) -> Result<(), u16> {
        if sample > 0x0fff {
            return Err(sample);
        }
        match channel {
            0 => {
                self.regs
                    .dac0data()
                    .write(|w| unsafe { w.data0().bits(sample) });
                Ok(())
            }
            1 => {
                self.regs
                    .dac1data()
                    .write(|w| unsafe { w.data1().bits(sample) });
                Ok(())
            }
            _ => Err(u16::from(channel)),
        }
    }

    /// Write both channels in one access (`DAC01DATA`).
    pub fn write_simultaneous(&self, channel0: u16, channel1: u16) -> Result<(), u16> {
        if channel0 > 0x0fff {
            return Err(channel0);
        }
        if channel1 > 0x0fff {
            return Err(channel1);
        }
        self.regs.dac01data().write(|w| unsafe {
            w.data0().bits(channel0);
            w.data1().bits(channel1);
            w
        });
        Ok(())
    }

    /// Nine 5-bit pattern steps. Values above 31 are rejected.
    pub fn set_pattern(&self, channel: u8, steps: &[u8; 9]) -> Result<(), u8> {
        if steps.iter().any(|step| *step > 31) {
            return Err(channel);
        }
        match channel {
            0 => {
                self.regs.dac0patl().write(|w| unsafe {
                    w.pat0().bits(steps[0]);
                    w.pat1().bits(steps[1]);
                    w.pat2().bits(steps[2]);
                    w.pat3().bits(steps[3]);
                    w.pat4().bits(steps[4]);
                    w.pat5().bits(steps[5]);
                    w
                });
                self.regs.dac0path().write(|w| unsafe {
                    w.pat6().bits(steps[6]);
                    w.pat7().bits(steps[7]);
                    w.pat8().bits(steps[8]);
                    w
                });
                Ok(())
            }
            1 => {
                self.regs.dac1patl().write(|w| unsafe {
                    w.pat0().bits(steps[0]);
                    w.pat1().bits(steps[1]);
                    w.pat2().bits(steps[2]);
                    w.pat3().bits(steps[3]);
                    w.pat4().bits(steps[4]);
                    w.pat5().bits(steps[5]);
                    w
                });
                self.regs.dac1path().write(|w| unsafe {
                    w.pat6().bits(steps[6]);
                    w.pat7().bits(steps[7]);
                    w.pat8().bits(steps[8]);
                    w
                });
                Ok(())
            }
            _ => Err(channel),
        }
    }

    /// Ramp start (`OFFS`) and step (`SCALE`).
    pub fn set_ramp(&self, channel: u8, start: u8, step: u8) -> Result<(), u8> {
        if step > 7 {
            return Err(step);
        }
        match channel {
            0 => {
                self.regs.dac0cfg1().modify(|_, w| unsafe {
                    w.offs().bits(start);
                    w.scale().bits(step);
                    w
                });
                Ok(())
            }
            1 => {
                self.regs.dac1cfg1().modify(|_, w| unsafe {
                    w.offs().bits(start);
                    w.scale().bits(step);
                    w
                });
                Ok(())
            }
            _ => Err(channel),
        }
    }

    pub fn software_trigger(&self, channel: u8) -> Result<(), u8> {
        match channel {
            0 => {
                self.regs.dac0cfg1().modify(|_, w| w.swtrig().set_bit());
                Ok(())
            }
            1 => {
                self.regs.dac1cfg1().modify(|_, w| w.swtrig().set_bit());
                Ok(())
            }
            _ => Err(channel),
        }
    }
}
