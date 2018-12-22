use core::cmp;
use cast::u32;
// use crate::device::scu::
use crate::device::{scu_clk};

use crate::time::Hertz;

/// Extension trait that constrains the `SCU` peripheral
pub trait ScuExt {
    /// Constrains the `SCU` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Scu;
}

/// Constrained SCU peripheral
pub struct Scu {
}


/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    hclk: Hertz,
    pclk1: Hertz,
    pclk2: Hertz,
    ppre1: u8,
    ppre2: u8,
    sysclk: Hertz,
    usbclk_valid: bool,
}

impl Clocks {
    /// Returns the frequency of the AHB
    pub fn hclk(&self) -> Hertz {
        self.hclk
    }

    /// Returns the frequency of the APB1
    pub fn pclk1(&self) -> Hertz {
        self.pclk1
    }

    /// Returns the frequency of the APB2
    pub fn pclk2(&self) -> Hertz {
        self.pclk2
    }

    pub(crate) fn ppre1(&self) -> u8 {
        self.ppre1
    }

    // TODO remove `allow`
    #[allow(dead_code)]
    pub(crate) fn ppre2(&self) -> u8 {
        self.ppre2
    }

    /// Returns the system (core) frequency
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    /// Returns whether the USBCLK clock frequency is valid for the USB peripheral
    pub fn usbclk_valid(&self) -> bool {
        self.usbclk_valid
    }
}