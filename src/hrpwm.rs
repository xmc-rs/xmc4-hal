//! High-resolution PWM.
//!
//! Present on XMC4100, XMC4200, and XMC4400. Each module has four high-resolution
//! channels and three comparator/slope generators.

use core::ops::Deref;

use crate::pac::{self, HRPWM0};
use crate::scu::{PeripheralClock, PeripheralReset, Scu};
use crate::sealed::Sealed;

pub trait HrpwmExt: Sealed {
    fn constrain(
        self,
        scu: &mut Scu,
        hrc: (
            pac::HRPWM0_HRC0,
            pac::HRPWM0_HRC1,
            pac::HRPWM0_HRC2,
            pac::HRPWM0_HRC3,
        ),
        csg: (pac::HRPWM0_CSG0, pac::HRPWM0_CSG1, pac::HRPWM0_CSG2),
    ) -> Hrpwm;
}

impl Sealed for HRPWM0 {}

impl HrpwmExt for HRPWM0 {
    fn constrain(
        self,
        scu: &mut Scu,
        hrc: (
            pac::HRPWM0_HRC0,
            pac::HRPWM0_HRC1,
            pac::HRPWM0_HRC2,
            pac::HRPWM0_HRC3,
        ),
        csg: (pac::HRPWM0_CSG0, pac::HRPWM0_CSG1, pac::HRPWM0_CSG2),
    ) -> Hrpwm {
        let pwm = Hrpwm {
            module: self,
            hrc: [(); 4],
            h0: hrc.0,
            h1: hrc.1,
            h2: hrc.2,
            h3: hrc.3,
            c0: csg.0,
            c1: csg.1,
            c2: csg.2,
        };
        pwm.enable(scu);
        pwm
    }
}

pub struct Hrpwm {
    module: HRPWM0,
    hrc: [(); 4],
    h0: pac::HRPWM0_HRC0,
    h1: pac::HRPWM0_HRC1,
    h2: pac::HRPWM0_HRC2,
    h3: pac::HRPWM0_HRC3,
    c0: pac::HRPWM0_CSG0,
    c1: pac::HRPWM0_CSG1,
    c2: pac::HRPWM0_CSG2,
}

impl Hrpwm {
    pub fn enable(&self, scu: &mut Scu) {
        scu.ungate_peripheral_clock(PeripheralClock::Hrpwm0);
        scu.deassert_peripheral_reset(PeripheralReset::Hrpwm0);
    }

    pub fn disable(&self, scu: &mut Scu) {
        scu.assert_peripheral_reset(PeripheralReset::Hrpwm0);
        scu.gate_peripheral_clock(PeripheralClock::Hrpwm0);
    }

    /// Turn on the high-resolution generation block (`GLBANA.GHREN`).
    pub fn enable_global_hr(&self) {
        self.module.glbana().modify(|_, w| w.ghren().set_bit());
    }

    pub fn disable_global_hr(&self) {
        self.module.glbana().modify(|_, w| w.ghren().clear_bit());
    }

    pub fn high_resolution_ready(&self) -> bool {
        self.module.hrghrs().read().hrgr().bit_is_set()
    }

    pub fn hrc(&self, index: usize) -> Option<Hrc<'_>> {
        let regs: &pac::hrpwm0_hrc0::RegisterBlock = match index {
            0 => &self.h0,
            1 => &self.h1,
            2 => &self.h2,
            3 => &self.h3,
            _ => return None,
        };
        Some(Hrc {
            index: index as u8,
            regs,
            module: &self.module,
        })
    }

    pub fn csg(&self, index: usize) -> Option<Csg<'_>> {
        let regs: &pac::hrpwm0_csg0::RegisterBlock = match index {
            0 => &self.c0,
            1 => &self.c1,
            2 => &self.c2,
            _ => return None,
        };
        Some(Csg {
            index: index as u8,
            regs,
            module: &self.module,
        })
    }
}

/// High-resolution channel. Compare and dead time are programmed through shadows.
pub struct Hrc<'a> {
    index: u8,
    regs: &'a pac::hrpwm0_hrc0::RegisterBlock,
    module: &'a pac::hrpwm0::RegisterBlock,
}

impl<'a> Hrc<'a> {
    pub fn enable(&self) {
        self.module.hrccfg().modify(|_, w| match self.index {
            0 => w.hrc0e().set_bit(),
            1 => w.hrc1e().set_bit(),
            2 => w.hrc2e().set_bit(),
            _ => w.hrc3e().set_bit(),
        });
    }

    pub fn disable(&self) {
        self.module.hrccfg().modify(|_, w| match self.index {
            0 => w.hrc0e().clear_bit(),
            1 => w.hrc1e().clear_bit(),
            2 => w.hrc2e().clear_bit(),
            _ => w.hrc3e().clear_bit(),
        });
    }

    /// Shadow rising and falling high-resolution compare values.
    pub fn set_compare(&self, rising: u8, falling: u8) {
        self.regs.scr1().write(|w| unsafe { w.scr1().bits(rising) });
        self.regs
            .scr2()
            .write(|w| unsafe { w.scr2().bits(falling) });
        self.transfer();
    }

    /// Shadow rising and falling dead time, in high-resolution ticks.
    pub fn set_dead_time(&self, rise: u16, fall: u16) {
        self.regs.sdcr().write(|w| unsafe { w.sdtrv().bits(rise) });
        self.regs.sdcf().write(|w| unsafe { w.sdtfv().bits(fall) });
        self.transfer();
    }

    fn transfer(&self) {
        self.module.hrcstrg().write(|w| match self.index {
            0 => {
                w.h0es().set_bit();
                w.h0des().set_bit()
            }
            1 => {
                w.h1es().set_bit();
                w.h1des().set_bit()
            }
            2 => {
                w.h2es().set_bit();
                w.h2des().set_bit()
            }
            _ => {
                w.h3es().set_bit();
                w.h3des().set_bit()
            }
        });
    }
}

/// Comparator and slope generator. The DAC reference is a 10-bit shadow value.
pub struct Csg<'a> {
    index: u8,
    regs: &'a pac::hrpwm0_csg0::RegisterBlock,
    module: &'a pac::hrpwm0::RegisterBlock,
}

impl<'a> Csg<'a> {
    /// Slope mode in `SC.SCM` (0 ..= 3).
    pub fn set_slope_mode(&self, mode: u8) -> Result<(), u8> {
        if mode > 3 {
            return Err(mode);
        }
        self.regs.sc().modify(|_, w| unsafe { w.scm().bits(mode) });
        Ok(())
    }

    pub fn start_slope(&self) {
        self.regs.sc().modify(|_, w| w.pse().set_bit());
    }

    pub fn stop_slope(&self) {
        self.regs.sc().modify(|_, w| w.pse().clear_bit());
    }

    /// DAC reference, 0 ..= 1023.
    pub fn set_dac(&self, value: u16) -> Result<(), u16> {
        if value > 0x3ff {
            return Err(value);
        }
        self.regs
            .sdsv1()
            .write(|w| unsafe { w.sdsv1().bits(value) });
        self.module.csgtrg().write(|w| match self.index {
            0 => w.d0svs().set_bit(),
            1 => w.d1svs().set_bit(),
            _ => w.d2svs().set_bit(),
        });
        Ok(())
    }

    pub fn dac(&self) -> u16 {
        self.regs.dsv1().read().dsv1().bits()
    }
}

impl Hrpwm {
    /// Silence the unused channel count so the four HRC slots stay obvious.
    pub fn channel_count(&self) -> usize {
        self.hrc.len()
    }
}

impl Deref for Hrpwm {
    type Target = HRPWM0;
    fn deref(&self) -> &Self::Target {
        &self.module
    }
}
