//! I2S on a USIC channel.

use core::ops::Deref;

use crate::pac;
use crate::scu::Clocks;
use crate::time::Hertz;
use crate::usic::{Channel, Mode};

pub struct I2s<C, CLK, WA, DOUT, DIN> {
    channel: Channel<C>,
    clk: CLK,
    wa: WA,
    dout: DOUT,
    din: DIN,
}

impl<C, CLK, WA, DOUT, DIN> I2s<C, CLK, WA, DOUT, DIN>
where
    C: Deref<Target = pac::usic0_ch0::RegisterBlock>,
{
    /// `bits` is the sample width (1..=16). The shift clock is `sclk`.
    pub fn new(
        channel: Channel<C>,
        clk: CLK,
        wa: WA,
        dout: DOUT,
        din: DIN,
        clocks: &Clocks,
        sclk: Hertz,
        bits: u8,
    ) -> Result<Self, u8> {
        if !(1..=16).contains(&bits) {
            return Err(bits);
        }
        channel.set_mode(Mode::I2s);
        channel
            .set_baudrate(clocks.pclk(), sclk, 1)
            .map_err(|_| bits)?;
        let length = bits - 1;
        channel.regs().sctr().modify(|_, w| unsafe {
            w.wle().bits(length);
            w.fle().bits(length);
            w.trm().bits(1);
            w
        });
        channel
            .regs()
            .tcsr()
            .modify(|_, w| unsafe { w.tden().bits(1) });
        channel.regs().pcr_iismode().modify(|_, w| {
            w.wagen().set_bit();
            w.mclk().set_bit();
            w
        });
        Ok(Self {
            channel,
            clk,
            wa,
            dout,
            din,
        })
    }

    pub fn start(&self) {
        self.channel.set_mode(Mode::I2s);
    }

    pub fn stop(&self) {
        self.channel.set_mode(Mode::Disabled);
    }

    /// Transmit one sample. `wa` 0 is the first slot, 1 is the second.
    ///
    /// The slot is selected by the transmit-buffer offset, the same way
    /// `XMC_I2S_CH_Transmit` indexes `TBUF`.
    pub fn transmit(&mut self, sample: u16, wa: u8) -> Result<(), u8> {
        let index = match wa {
            0 => 0,
            1 => 16,
            _ => return Err(wa),
        };
        while self.channel.regs().tcsr().read().tdv().bit_is_set() {}
        self.channel
            .regs()
            .tbuf(index)
            .write(|w| unsafe { w.tdata().bits(sample) });
        Ok(())
    }

    pub fn receive(&self) -> u16 {
        self.channel.read_rx()
    }

    pub fn free(self) -> (Channel<C>, CLK, WA, DOUT, DIN) {
        (self.channel, self.clk, self.wa, self.dout, self.din)
    }
}
