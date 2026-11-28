//! USIC channel layer shared by UART, SPI, I2C, and I2S.
//!
//! The kernel clock is enabled on the module. Each channel then selects a
//! protocol, a baud rate, a FIFO, and the DXCR input stage.

use core::ops::Deref;

use crate::pac::{self, USIC0, USIC0_CH0, USIC0_CH1, USIC1, USIC1_CH0, USIC1_CH1};
#[cfg(not(feature = "xmc4500"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;
use crate::time::Hertz;

#[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
use crate::pac::{USIC2, USIC2_CH0, USIC2_CH1};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Mode {
    Disabled = 0,
    Spi = 1,
    Uart = 2,
    I2s = 3,
    I2c = 4,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Kernel {
    Usic0,
    Usic1,
    #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
    Usic2,
}

pub trait UsicExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Usic<Self>
    where
        Self: Sized;
}

impl Sealed for USIC0 {}
impl Sealed for USIC1 {}

impl UsicExt for USIC0 {
    fn constrain(self, scu: &mut Scu) -> Usic<Self> {
        let usic = Usic {
            module: self,
            kernel: Kernel::Usic0,
        };
        usic.enable(scu);
        usic
    }
}

impl UsicExt for USIC1 {
    fn constrain(self, scu: &mut Scu) -> Usic<Self> {
        let usic = Usic {
            module: self,
            kernel: Kernel::Usic1,
        };
        usic.enable(scu);
        usic
    }
}

#[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
impl Sealed for USIC2 {}

#[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
impl UsicExt for USIC2 {
    fn constrain(self, scu: &mut Scu) -> Usic<Self> {
        let usic = Usic {
            module: self,
            kernel: Kernel::Usic2,
        };
        usic.enable(scu);
        usic
    }
}

pub struct Usic<M> {
    module: M,
    kernel: Kernel,
}

impl<M> Usic<M> {
    pub fn enable(&self, scu: &mut Scu) {
        match self.kernel {
            Kernel::Usic0 => {
                #[cfg(not(feature = "xmc4500"))]
                scu.ungate_peripheral_clock(PeripheralClock::Usic0);
                scu.deassert_peripheral_reset(PeripheralReset::Usic0);
            }
            Kernel::Usic1 => {
                #[cfg(not(feature = "xmc4500"))]
                scu.ungate_peripheral_clock(PeripheralClock::Usic1);
                scu.deassert_peripheral_reset(PeripheralReset::Usic1);
            }
            #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
            Kernel::Usic2 => {
                #[cfg(not(feature = "xmc4500"))]
                scu.ungate_peripheral_clock(PeripheralClock::Usic2);
                scu.deassert_peripheral_reset(PeripheralReset::Usic2);
            }
        }
    }

    pub fn disable(&self, scu: &mut Scu) {
        match self.kernel {
            Kernel::Usic0 => {
                scu.assert_peripheral_reset(PeripheralReset::Usic0);
                #[cfg(not(feature = "xmc4500"))]
                scu.gate_peripheral_clock(PeripheralClock::Usic0);
            }
            Kernel::Usic1 => {
                scu.assert_peripheral_reset(PeripheralReset::Usic1);
                #[cfg(not(feature = "xmc4500"))]
                scu.gate_peripheral_clock(PeripheralClock::Usic1);
            }
            #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
            Kernel::Usic2 => {
                scu.assert_peripheral_reset(PeripheralReset::Usic2);
                #[cfg(not(feature = "xmc4500"))]
                scu.gate_peripheral_clock(PeripheralClock::Usic2);
            }
        }
    }

    pub fn channel<C>(&self, channel: C) -> Channel<C>
    where
        C: Deref<Target = pac::usic0_ch0::RegisterBlock>,
    {
        Channel::new(channel)
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
                Channel::new(self)
            }
        }
    )+};
}

channel_ext!(USIC0_CH0, USIC0_CH1, USIC1_CH0, USIC1_CH1);
#[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
channel_ext!(USIC2_CH0, USIC2_CH1);

/// One USIC channel.
pub struct Channel<C> {
    regs: C,
}

impl<C> Channel<C>
where
    C: Deref<Target = pac::usic0_ch0::RegisterBlock>,
{
    pub fn new(regs: C) -> Self {
        let channel = Self { regs };
        channel.enable();
        channel
    }

    /// Set `KSCFG.MODEN` together with its write-enable bit.
    pub fn enable(&self) {
        self.regs.kscfg().write(|w| {
            w.moden().set_bit();
            w.bpmoden().set_bit();
            w
        });
    }

    pub fn disable(&self) {
        self.regs.kscfg().write(|w| {
            w.moden().clear_bit();
            w.bpmoden().set_bit();
            w
        });
    }

    pub fn set_mode(&self, mode: Mode) {
        self.regs
            .ccr()
            .modify(|_, w| unsafe { w.mode().bits(mode as u8) });
    }

    /// Fractional baud generator from `XMC_USIC_CH_SetBaudrate`.
    ///
    /// `kernel` is the peripheral clock (`Clocks::pclk`). `oversampling` is the
    /// number of samples per bit (16 for UART, 2 for SPI is typical).
    pub fn set_baudrate(&self, kernel: Hertz, rate: Hertz, oversampling: u32) -> Result<(), Hertz> {
        if rate.0 < 100 || oversampling == 0 {
            return Err(rate);
        }
        let peripheral_clock = kernel.0 / 100;
        let rate_div = rate.0 / 100;
        let mut clock_divider_min = 1024u32;
        let mut pdiv_int_min = 1u32;
        let mut pdiv_frac_min = 0x3ffu32;
        let mut found = false;

        for clock_divider in (1..=1024).rev() {
            let pdiv = (peripheral_clock.saturating_mul(clock_divider))
                / rate_div.saturating_mul(oversampling).max(1);
            let pdiv_int = pdiv >> 10;
            let pdiv_frac = pdiv & 0x3ff;
            if (1..=1024).contains(&pdiv_int) && pdiv_frac < pdiv_frac_min {
                pdiv_frac_min = pdiv_frac;
                pdiv_int_min = pdiv_int;
                clock_divider_min = clock_divider;
                found = true;
            }
        }
        if !found {
            return Err(rate);
        }

        self.regs.fdr().write(|w| unsafe {
            w.dm().bits(2);
            w.step().bits((clock_divider_min - 1) as u16);
            w
        });
        self.regs.brg().modify(|_, w| unsafe {
            w.dctq().bits((oversampling - 1) as u8);
            w.pdiv().bits((pdiv_int_min - 1) as u16);
            w
        });
        Ok(())
    }

    /// Input stage `dx` (0..=5) selects source `dsel` (0..=7).
    pub fn set_input(&self, dx: u8, dsel: u8) -> Result<(), u8> {
        if dsel > 7 {
            return Err(dsel);
        }
        match dx {
            0 => {
                self.regs
                    .dx0cr()
                    .modify(|_, w| unsafe { w.dsel().bits(dsel) });
            }
            1 => {
                self.regs
                    .dx1cr()
                    .modify(|_, w| unsafe { w.dsel().bits(dsel) });
            }
            2 => {
                self.regs
                    .dx2cr()
                    .modify(|_, w| unsafe { w.dsel().bits(dsel) });
            }
            3 => {
                self.regs
                    .dx3cr()
                    .modify(|_, w| unsafe { w.dsel().bits(dsel) });
            }
            4 => {
                self.regs
                    .dx4cr()
                    .modify(|_, w| unsafe { w.dsel().bits(dsel) });
            }
            5 => {
                self.regs
                    .dx5cr()
                    .modify(|_, w| unsafe { w.dsel().bits(dsel) });
            }
            _ => return Err(dx),
        }
        Ok(())
    }

    /// Transmit FIFO size code (0..=7) and limit. The size code is `log2(words)`.
    pub fn configure_tx_fifo(&self, start: u8, size: u8, limit: u8) -> Result<(), u8> {
        if size > 7 {
            return Err(size);
        }
        self.regs.tbctr().modify(|_, w| unsafe {
            w.dptr().bits(start);
            w.size().bits(size);
            w.limit().bits(limit);
            w
        });
        Ok(())
    }

    pub fn configure_rx_fifo(&self, start: u8, size: u8, limit: u8) -> Result<(), u8> {
        if size > 7 {
            return Err(size);
        }
        self.regs.rbctr().modify(|_, w| unsafe {
            w.dptr().bits(start);
            w.size().bits(size);
            w.limit().bits(limit);
            w
        });
        Ok(())
    }

    pub fn write_tx(&self, word: u16) {
        self.regs.tbuf(0).write(|w| unsafe { w.tdata().bits(word) });
    }

    pub fn read_rx(&self) -> u16 {
        self.regs.rbuf().read().dsr().bits()
    }

    pub fn regs(&self) -> &pac::usic0_ch0::RegisterBlock {
        &self.regs
    }
}
