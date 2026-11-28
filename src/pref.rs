//! Flash prefetch buffer.

use cortex_m::asm::{dsb, isb};

use crate::pac::PREF;
use crate::sealed::Sealed;

pub trait PrefExt: Sealed {
    fn constrain(self) -> Pref;
}

impl Sealed for PREF {}

impl PrefExt for PREF {
    fn constrain(self) -> Pref {
        Pref::new(self)
    }
}

pub struct Pref {
    pub regs: PREF,
}

impl Pref {
    pub fn new(pref: PREF) -> Self {
        Self { regs: pref }
    }

    /// Cache instruction fetches in the prefetch buffer.
    pub fn enable_instruction_buffer(&self) {
        self.regs.pcon().modify(|_, w| w.ibyp().clear_bit());
    }

    /// Bypass the instruction buffer.
    pub fn disable_instruction_buffer(&self) {
        self.regs.pcon().modify(|_, w| w.ibyp().set_bit());
    }

    /// Drop every line in the instruction buffer.
    ///
    /// The invalidate bit is pulsed, with a data and instruction barrier on
    /// each edge, matching `XMC_PREFETCH_InvalidateInstructionBuffer`.
    pub fn invalidate_instruction_buffer(&self) {
        self.regs.pcon().modify(|_, w| w.iinv().set_bit());
        dsb();
        isb();
        self.regs.pcon().modify(|_, w| w.iinv().clear_bit());
        dsb();
        isb();
    }

    /// Cache data fetches in the prefetch buffer.
    ///
    /// XMC4400 and XMC4500 prefetch blocks have no data-buffer bypass bit.
    #[cfg(not(any(feature = "xmc4400", feature = "xmc4500")))]
    pub fn enable_data_buffer(&self) {
        self.regs.pcon().modify(|_, w| w.dbyp().clear_bit());
    }

    /// Bypass the data buffer.
    #[cfg(not(any(feature = "xmc4400", feature = "xmc4500")))]
    pub fn disable_data_buffer(&self) {
        self.regs.pcon().modify(|_, w| w.dbyp().set_bit());
    }
}
