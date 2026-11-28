//! External bus unit.
//!
//! Region select, SDRAM mode, and refresh. Compiled for devices that have EBU.

use crate::pac::EBU;
#[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;

pub trait EbuExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Ebu;
}

impl Sealed for EBU {}

impl EbuExt for EBU {
    fn constrain(self, scu: &mut Scu) -> Ebu {
        let ebu = Ebu { regs: self };
        ebu.enable(scu);
        ebu
    }
}

pub struct Ebu {
    regs: EBU,
}

impl Ebu {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
        scu.ungate_peripheral_clock(PeripheralClock::Ebu);
        scu.deassert_peripheral_reset(PeripheralReset::Ebu);
        self.regs.clc().modify(|_, w| w.disr().clear_bit());
    }

    pub fn disable(&self, scu: &mut Scu) {
        self.regs.clc().modify(|_, w| w.disr().set_bit());
        scu.assert_peripheral_reset(PeripheralReset::Ebu);
        #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
        scu.gate_peripheral_clock(PeripheralClock::Ebu);
    }

    /// Program one address region. `base_select` is the ADDRSEL base and mask in bits 31:4.
    pub fn set_region(
        &self,
        region: u8,
        base_select: u32,
        enable: bool,
        write_protect: bool,
    ) -> Result<(), u8> {
        let mut value = base_select & !0b111;
        if enable {
            value |= 1;
        }
        if write_protect {
            value |= 1 << 2;
        }
        match region {
            0 => self.regs.addrsel0().write(|w| unsafe { w.bits(value) }),
            1 => self.regs.addrsel1().write(|w| unsafe { w.bits(value) }),
            2 => self.regs.addrsel2().write(|w| unsafe { w.bits(value) }),
            3 => self.regs.addrsel3().write(|w| unsafe { w.bits(value) }),
            _ => return Err(region),
        };
        Ok(())
    }

    pub fn set_read_wait(&self, region: u8, cycles: u8) -> Result<(), u8> {
        match region {
            0 => self
                .regs
                .busrcon0()
                .modify(|_, w| unsafe { w.wait().bits(cycles) }),
            1 => self
                .regs
                .busrcon1()
                .modify(|_, w| unsafe { w.wait().bits(cycles) }),
            2 => self
                .regs
                .busrcon2()
                .modify(|_, w| unsafe { w.wait().bits(cycles) }),
            3 => self
                .regs
                .busrcon3()
                .modify(|_, w| unsafe { w.wait().bits(cycles) }),
            _ => return Err(region),
        };
        Ok(())
    }

    /// SDRAM geometry and CAS latency, then a cold start of the mode register.
    pub fn configure_sdram(&self, cas_latency: u8, banks: u8, width: u8) -> Result<(), u8> {
        if cas_latency > 7 || banks > 7 || width > 3 {
            return Err(cas_latency);
        }
        self.regs.sdrmcon().modify(|_, w| unsafe {
            w.bankm().bits(banks);
            w.awidth().bits(width);
            w
        });
        self.regs.sdrmod().modify(|_, w| unsafe {
            w.caslat().bits(cas_latency);
            w.coldstart().set_bit();
            w
        });
        Ok(())
    }

    pub fn set_refresh(&self, period: u8) {
        self.regs
            .sdrmref()
            .modify(|_, w| unsafe { w.refreshc().bits(period) });
    }

    pub fn enable_auto_refresh(&self) {
        self.regs.sdrmref().modify(|_, w| w.arfsh().set_bit());
    }

    pub fn disable_auto_refresh(&self) {
        self.regs.sdrmref().modify(|_, w| w.arfsh().clear_bit());
    }
}
