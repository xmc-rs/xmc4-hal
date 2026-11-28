//! General-purpose DMA.
//!
//! A channel is programmed with source, destination, and block size, then
//! started with a software request. `GPDMA1` exists only on the devices that
//! have that controller.

use crate::pac::{
    GPDMA0, GPDMA0_CH0, GPDMA0_CH1, GPDMA0_CH2, GPDMA0_CH3, GPDMA0_CH4, GPDMA0_CH5, GPDMA0_CH6,
    GPDMA0_CH7,
};
#[cfg(not(feature = "xmc4500"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;

#[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
use crate::pac::{GPDMA1, GPDMA1_CH0, GPDMA1_CH1, GPDMA1_CH2, GPDMA1_CH3};

pub trait GpdmaExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Dma<Self>
    where
        Self: Sized;
}

impl Sealed for GPDMA0 {}

impl GpdmaExt for GPDMA0 {
    fn constrain(self, scu: &mut Scu) -> Dma<Self> {
        let dma = Dma { regs: self };
        dma.enable(scu);
        dma
    }
}

#[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
impl Sealed for GPDMA1 {}

#[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
impl GpdmaExt for GPDMA1 {
    fn constrain(self, scu: &mut Scu) -> Dma<Self> {
        let dma = Dma { regs: self };
        dma.enable(scu);
        dma
    }
}

pub struct Dma<R> {
    regs: R,
}

impl Dma<GPDMA0> {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(not(feature = "xmc4500"))]
        scu.ungate_peripheral_clock(PeripheralClock::Dma0);
        scu.deassert_peripheral_reset(PeripheralReset::Gpdma0);
        self.regs.dmacfgreg().modify(|_, w| w.dma_en().set_bit());
    }

    pub fn disable(&self, scu: &mut Scu) {
        self.regs.dmacfgreg().modify(|_, w| w.dma_en().clear_bit());
        scu.assert_peripheral_reset(PeripheralReset::Gpdma0);
        #[cfg(not(feature = "xmc4500"))]
        scu.gate_peripheral_clock(PeripheralClock::Dma0);
    }

    pub fn enable_channel(&self, channel: u8) -> Result<(), u8> {
        self.set_channel_enable(channel, true)
    }

    pub fn disable_channel(&self, channel: u8) -> Result<(), u8> {
        self.set_channel_enable(channel, false)
    }

    fn set_channel_enable(&self, channel: u8, enable: bool) -> Result<(), u8> {
        if channel > 7 {
            return Err(channel);
        }
        let bit = 1u8 << channel;
        self.regs.chenreg().write(|w| unsafe {
            w.we_ch().bits(bit);
            w.ch().bits(if enable { bit } else { 0 });
            w
        });
        Ok(())
    }

    pub fn software_trigger(&self, channel: u8) -> Result<(), u8> {
        if channel > 7 {
            return Err(channel);
        }
        self.regs.sglreqsrcreg().modify(|_, w| {
            match channel {
                0 => {
                    w.we_ch0().set_bit();
                    w.ch0().set_bit();
                }
                1 => {
                    w.we_ch1().set_bit();
                    w.ch1().set_bit();
                }
                2 => {
                    w.we_ch2().set_bit();
                    w.ch2().set_bit();
                }
                3 => {
                    w.we_ch3().set_bit();
                    w.ch3().set_bit();
                }
                4 => {
                    w.we_ch4().set_bit();
                    w.ch4().set_bit();
                }
                5 => {
                    w.we_ch5().set_bit();
                    w.ch5().set_bit();
                }
                6 => {
                    w.we_ch6().set_bit();
                    w.ch6().set_bit();
                }
                _ => {
                    w.we_ch7().set_bit();
                    w.ch7().set_bit();
                }
            }
            w
        });
        Ok(())
    }

    pub fn transfer_complete(&self, channel: u8) -> Result<bool, u8> {
        if channel > 7 {
            return Err(channel);
        }
        let status = self.regs.statustfr().read();
        Ok(match channel {
            0 => status.ch0().bit_is_set(),
            1 => status.ch1().bit_is_set(),
            2 => status.ch2().bit_is_set(),
            3 => status.ch3().bit_is_set(),
            4 => status.ch4().bit_is_set(),
            5 => status.ch5().bit_is_set(),
            6 => status.ch6().bit_is_set(),
            _ => status.ch7().bit_is_set(),
        })
    }
}

#[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
impl Dma<GPDMA1> {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(not(feature = "xmc4500"))]
        scu.ungate_peripheral_clock(PeripheralClock::Dma1);
        scu.deassert_peripheral_reset(PeripheralReset::Gpdma1);
        self.regs.dmacfgreg().modify(|_, w| w.dma_en().set_bit());
    }

    pub fn disable(&self, scu: &mut Scu) {
        self.regs.dmacfgreg().modify(|_, w| w.dma_en().clear_bit());
        scu.assert_peripheral_reset(PeripheralReset::Gpdma1);
        #[cfg(not(feature = "xmc4500"))]
        scu.gate_peripheral_clock(PeripheralClock::Dma1);
    }
}

macro_rules! dma_channel {
    ($($ty:ty),+) => {$(
        impl Channel<$ty> {
            pub fn new(regs: $ty) -> Self {
                Self { regs }
            }

            pub fn set_addresses(&self, source: u32, destination: u32) {
                self.regs.sar().write(|w| unsafe { w.sar().bits(source) });
                self.regs.dar().write(|w| unsafe { w.dar().bits(destination) });
            }

            /// Block size in transfer beats. The controller stores `beats`.
            pub fn set_block_size(&self, beats: u16) -> Result<(), u16> {
                if beats > 0x0fff {
                    return Err(beats);
                }
                self.regs
                    .ctlh()
                    .write(|w| unsafe { w.block_ts().bits(beats) });
                Ok(())
            }

            /// Memory-to-memory style handshake: both sides are software-selected.
            pub fn software_handshake(&self) {
                self.regs.cfgl().modify(|_, w| {
                    w.hs_sel_src().set_bit();
                    w.hs_sel_dst().set_bit();
                    w
                });
            }
        }
    )+};
}

pub struct Channel<C> {
    regs: C,
}

dma_channel!(
    GPDMA0_CH0, GPDMA0_CH1, GPDMA0_CH2, GPDMA0_CH3, GPDMA0_CH4, GPDMA0_CH5, GPDMA0_CH6, GPDMA0_CH7
);

#[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
dma_channel!(GPDMA1_CH0, GPDMA1_CH1, GPDMA1_CH2, GPDMA1_CH3);
