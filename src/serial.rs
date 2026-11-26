//! UART on a USIC channel (ASC mode).

use core::convert::Infallible;
use core::ops::Deref;

use embedded_hal_nb::serial::{ErrorType, Read, Write};

use crate::pac;
use crate::scu::Clocks;
use crate::time::Bps;
use crate::usic::{Channel, Mode};

pub struct Serial<C, TX, RX> {
    channel: Channel<C>,
    tx: TX,
    rx: RX,
}

impl<C, TX, RX> Serial<C, TX, RX>
where
    C: Deref<Target = pac::usic0_ch0::RegisterBlock>,
{
    /// 8-bit frames, no parity. `tx` and `rx` are consumed so their alternate
    /// function cannot be used by another driver.
    pub fn new(
        channel: Channel<C>,
        tx: TX,
        rx: RX,
        clocks: &Clocks,
        baud: Bps,
    ) -> Result<Self, Bps> {
        channel.set_mode(Mode::Uart);
        channel
            .set_baudrate(clocks.pclk(), crate::time::Hertz(baud.0), 16)
            .map_err(|_| baud)?;
        channel.regs().sctr().modify(|_, w| unsafe {
            w.wle().bits(7);
            w.fle().bits(7);
            w.trm().bits(1);
            w
        });
        channel
            .regs()
            .tcsr()
            .modify(|_, w| unsafe { w.tden().bits(1) });
        Ok(Self { channel, tx, rx })
    }

    pub fn free(self) -> (Channel<C>, TX, RX) {
        (self.channel, self.tx, self.rx)
    }

    pub fn write_blocking(&mut self, byte: u8) {
        nb::block!(Write::write(self, byte)).ok();
    }

    pub fn read_blocking(&mut self) -> u8 {
        nb::block!(Read::read(self)).unwrap_or(0)
    }
}

impl<C, TX, RX> ErrorType for Serial<C, TX, RX> {
    type Error = Infallible;
}

impl<C, TX, RX> Read<u8> for Serial<C, TX, RX>
where
    C: Deref<Target = pac::usic0_ch0::RegisterBlock>,
{
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let status = self.channel.regs().psr_ascmode().read();
        if status.rif().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        }
        let byte = self.channel.read_rx() as u8;
        self.channel.regs().pscr().write(|w| w.crif().set_bit());
        Ok(byte)
    }
}

impl<C, TX, RX> Write<u8> for Serial<C, TX, RX>
where
    C: Deref<Target = pac::usic0_ch0::RegisterBlock>,
{
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if self.channel.regs().tcsr().read().tdv().bit_is_set() {
            return Err(nb::Error::WouldBlock);
        }
        self.channel.write_tx(u16::from(word));
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.channel.regs().tcsr().read().tdv().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}
