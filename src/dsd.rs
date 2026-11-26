//! Delta-sigma demodulator.
//!
//! Clock, channel result, main and auxiliary filters, the integrator
//! timestamp, and the waveform generator. Compiled for devices that have DSD.

use core::ops::Deref;

use crate::pac::{self, DSD, DSD_CH0, DSD_CH1, DSD_CH2, DSD_CH3};
#[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;

pub trait DsdExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Dsd;
}

impl Sealed for DSD {}

impl DsdExt for DSD {
    fn constrain(self, scu: &mut Scu) -> Dsd {
        let dsd = Dsd { regs: self };
        dsd.enable(scu);
        dsd
    }
}

pub struct Dsd {
    regs: DSD,
}

impl Dsd {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
        scu.ungate_peripheral_clock(PeripheralClock::Dsd);
        scu.deassert_peripheral_reset(PeripheralReset::Dsd);
        self.regs.clc().modify(|_, w| w.disr().clear_bit());
    }

    pub fn disable(&self, scu: &mut Scu) {
        self.regs.clc().modify(|_, w| w.disr().set_bit());
        scu.assert_peripheral_reset(PeripheralReset::Dsd);
        #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
        scu.gate_peripheral_clock(PeripheralClock::Dsd);
    }

    pub fn set_divider(&self, divider: u8) {
        self.regs
            .cgcfg()
            .modify(|_, w| unsafe { w.divcg().bits(divider) });
    }
}

pub trait ChannelExt: Sealed {
    fn constrain(self) -> Channel<Self>
    where
        Self: Sized;
}

macro_rules! channel_ext {
    ($($ty:ty),+) => {$(
        impl Sealed for $ty {}
        impl ChannelExt for $ty {
            fn constrain(self) -> Channel<Self> {
                Channel { regs: self }
            }
        }
    )+};
}

channel_ext!(DSD_CH0, DSD_CH1, DSD_CH2, DSD_CH3);

pub struct Channel<C> {
    regs: C,
}

impl<C> Channel<C>
where
    C: Deref<Target = pac::dsd_ch0::RegisterBlock>,
{
    pub fn enable_main_filter(&self, decimation: u8) {
        self.regs.fcfgc().modify(|_, w| unsafe {
            w.cfmdf().bits(decimation);
            w.cfen().set_bit();
            w
        });
    }

    pub fn disable_main_filter(&self) {
        self.regs.fcfgc().modify(|_, w| w.cfen().clear_bit());
    }

    pub fn set_aux_decimation(&self, decimation: u8) {
        self.regs
            .fcfga()
            .modify(|_, w| unsafe { w.cfadf().bits(decimation) });
    }

    pub fn enable_integrator(&self) {
        self.regs.iwctr().modify(|_, w| w.iws().set_bit());
    }

    pub fn disable_integrator(&self) {
        self.regs.iwctr().modify(|_, w| w.iws().clear_bit());
    }

    pub fn enable_waveform(&self) {
        self.regs.rectcfg().modify(|_, w| w.rfen().set_bit());
    }

    pub fn disable_waveform(&self) {
        self.regs.rectcfg().modify(|_, w| w.rfen().clear_bit());
    }

    pub fn result(&self) -> u16 {
        self.regs.resm().read().result().bits()
    }

    pub fn aux_result(&self) -> u16 {
        self.regs.resa().read().result().bits()
    }

    pub fn timestamp(&self) -> u16 {
        self.regs.tstmp().read().result().bits()
    }
}
