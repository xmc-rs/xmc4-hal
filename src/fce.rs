//! Flexible CRC engine.
//!
//! The four kernels share one register block. Kernel 0 is CRC-32, kernel 1 is
//! CRC-16, and kernels 2 and 3 are CRC-8. The seed is written to `CRC`, input
//! words go to `IR`, and the running result is read from `RES`.

use core::ops::Deref;

use crate::pac::{self, FCE, FCE_KE0, FCE_KE1, FCE_KE2, FCE_KE3};
#[cfg(not(feature = "xmc4500"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;

pub trait FceExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Fce;
}

impl Sealed for FCE {}

impl FceExt for FCE {
    fn constrain(self, scu: &mut Scu) -> Fce {
        let fce = Fce { regs: self };
        fce.enable(scu);
        fce
    }
}

pub struct Fce {
    regs: FCE,
}

impl Fce {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(not(feature = "xmc4500"))]
        scu.ungate_peripheral_clock(PeripheralClock::Fce);
        scu.deassert_peripheral_reset(PeripheralReset::Fce);
        self.regs.clc().modify(|_, w| w.disr().clear_bit());
    }

    pub fn disable(&self, scu: &mut Scu) {
        self.regs.clc().modify(|_, w| w.disr().set_bit());
        scu.assert_peripheral_reset(PeripheralReset::Fce);
        #[cfg(not(feature = "xmc4500"))]
        scu.gate_peripheral_clock(PeripheralClock::Fce);
    }
}

pub trait KernelExt: Sealed {
    fn constrain(self) -> Kernel<Self>
    where
        Self: Sized;
}

macro_rules! kernel_ext {
    ($($ty:ty),+) => {$(
        impl Sealed for $ty {}
        impl KernelExt for $ty {
            fn constrain(self) -> Kernel<Self> {
                Kernel { regs: self }
            }
        }
    )+};
}

kernel_ext!(FCE_KE0, FCE_KE1, FCE_KE2, FCE_KE3);

pub struct Kernel<K> {
    regs: K,
}

impl<K> Kernel<K>
where
    K: Deref<Target = pac::fce_ke0::RegisterBlock>,
{
    pub fn seed(&self, value: u32) {
        self.regs.crc().write(|w| unsafe { w.crc().bits(value) });
    }

    pub fn set_length(&self, bytes: u16) {
        self.regs
            .length()
            .write(|w| unsafe { w.length().bits(bytes) });
    }

    pub fn reflect(&self, input: bool, output: bool) {
        self.regs.cfg().modify(|_, w| {
            if input {
                w.refin().set_bit();
            } else {
                w.refin().clear_bit();
            }
            if output {
                w.refout().set_bit();
            } else {
                w.refout().clear_bit();
            }
            w
        });
    }

    pub fn push_word(&self, word: u32) {
        self.regs.ir().write(|w| unsafe { w.ir().bits(word) });
    }

    pub fn push_byte(&self, byte: u8) {
        self.push_word(u32::from(byte));
    }

    /// CRC of `data`, one byte at a time. Suitable for the CRC-8 kernels.
    pub fn crc8(&self, seed: u8, data: &[u8]) -> u8 {
        self.seed(u32::from(seed));
        for byte in data {
            self.push_byte(*byte);
        }
        self.result() as u8
    }

    /// CRC of `data` as little-endian halfwords. Suitable for the CRC-16 kernel.
    pub fn crc16(&self, seed: u16, data: &[u16]) -> u16 {
        self.seed(u32::from(seed));
        for word in data {
            self.push_word(u32::from(*word));
        }
        self.result() as u16
    }

    /// CRC of `data` as 32-bit words. Suitable for the CRC-32 kernel.
    pub fn crc32(&self, seed: u32, data: &[u32]) -> u32 {
        self.seed(seed);
        for word in data {
            self.push_word(*word);
        }
        self.result()
    }

    pub fn result(&self) -> u32 {
        self.regs.res().read().res().bits()
    }
}
