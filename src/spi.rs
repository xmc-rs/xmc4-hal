//! SPI master on a USIC channel (SSC mode).

use core::convert::Infallible;
use core::ops::Deref;

use embedded_hal::spi::{ErrorType, SpiBus};
use embedded_hal_02::blocking::spi::{Transfer, Write as SpiWrite02};

use crate::pac;
use crate::scu::Clocks;
use crate::time::Hertz;
use crate::usic::{Channel, Mode};

pub struct Spi<C, SCK, MOSI, MISO> {
    channel: Channel<C>,
    sck: SCK,
    mosi: MOSI,
    miso: MISO,
}

impl<C, SCK, MOSI, MISO> Spi<C, SCK, MOSI, MISO>
where
    C: Deref<Target = pac::usic0_ch0::RegisterBlock>,
{
    /// 8-bit master. The pins are consumed in their alternate-function mode.
    pub fn new(
        channel: Channel<C>,
        sck: SCK,
        mosi: MOSI,
        miso: MISO,
        clocks: &Clocks,
        freq: Hertz,
    ) -> Result<Self, Hertz> {
        channel.set_mode(Mode::Spi);
        channel.set_baudrate(clocks.pclk(), freq, 2)?;
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
        channel.regs().pcr_sscmode().modify(|_, w| {
            w.mclk().set_bit();
            w.selctr().set_bit();
            w
        });
        Ok(Self {
            channel,
            sck,
            mosi,
            miso,
        })
    }

    pub fn free(self) -> (Channel<C>, SCK, MOSI, MISO) {
        (self.channel, self.sck, self.mosi, self.miso)
    }

    fn exchange(&mut self, byte: u8) -> u8 {
        while self.channel.regs().tcsr().read().tdv().bit_is_set() {}
        self.channel.regs().pscr().write(|w| w.crif().set_bit());
        self.channel.write_tx(u16::from(byte));
        while self
            .channel
            .regs()
            .psr_sscmode()
            .read()
            .rif()
            .bit_is_clear()
        {}
        self.channel.read_rx() as u8
    }
}

impl<C, SCK, MOSI, MISO> ErrorType for Spi<C, SCK, MOSI, MISO> {
    type Error = Infallible;
}

impl<C, SCK, MOSI, MISO> SpiBus<u8> for Spi<C, SCK, MOSI, MISO>
where
    C: Deref<Target = pac::usic0_ch0::RegisterBlock>,
{
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for word in words {
            *word = self.exchange(0);
        }
        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        for word in words {
            let _ = self.exchange(*word);
        }
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let len = read.len().max(write.len());
        for index in 0..len {
            let out = write.get(index).copied().unwrap_or(0);
            let input = self.exchange(out);
            if let Some(slot) = read.get_mut(index) {
                *slot = input;
            }
        }
        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for word in words {
            *word = self.exchange(*word);
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        while self.channel.regs().tcsr().read().tdv().bit_is_set() {}
        Ok(())
    }
}

impl<C, SCK, MOSI, MISO> Transfer<u8> for Spi<C, SCK, MOSI, MISO>
where
    C: Deref<Target = pac::usic0_ch0::RegisterBlock>,
{
    type Error = Infallible;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        SpiBus::transfer_in_place(self, words)?;
        Ok(words)
    }
}

impl<C, SCK, MOSI, MISO> SpiWrite02<u8> for Spi<C, SCK, MOSI, MISO>
where
    C: Deref<Target = pac::usic0_ch0::RegisterBlock>,
{
    type Error = Infallible;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        SpiBus::write(self, words)
    }
}
