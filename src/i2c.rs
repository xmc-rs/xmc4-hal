//! I2C master on a USIC channel.

use core::ops::Deref;

use embedded_hal::i2c::{
    ErrorKind, ErrorType, I2c, NoAcknowledgeSource, Operation, SevenBitAddress,
};

use crate::pac;
use crate::scu::Clocks;
use crate::time::Hertz;
use crate::usic::{Channel, Mode};

const MASTER_SEND: u16 = 0;
const MASTER_RECEIVE_ACK: u16 = 2 << 8;
const MASTER_RECEIVE_NACK: u16 = 3 << 8;
const MASTER_START: u16 = 4 << 8;
const MASTER_RESTART: u16 = 5 << 8;
const MASTER_STOP: u16 = 6 << 8;

pub struct I2cMaster<C, SCL, SDA> {
    channel: Channel<C>,
    scl: SCL,
    sda: SDA,
}

impl<C, SCL, SDA> I2cMaster<C, SCL, SDA>
where
    C: Deref<Target = pac::usic0_ch0::RegisterBlock>,
{
    /// Standard-mode master (10x oversampling). Pins are consumed.
    pub fn new(
        channel: Channel<C>,
        scl: SCL,
        sda: SDA,
        clocks: &Clocks,
        freq: Hertz,
    ) -> Result<Self, Hertz> {
        channel.set_mode(Mode::I2c);
        channel.set_baudrate(clocks.pclk(), freq, 10)?;
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
        Ok(Self { channel, scl, sda })
    }

    pub fn free(self) -> (Channel<C>, SCL, SDA) {
        (self.channel, self.scl, self.sda)
    }

    fn wait_tx(&self) {
        while self.channel.regs().tcsr().read().tdv().bit_is_set() {}
    }

    fn command(&mut self, word: u16) -> Result<(), ErrorKind> {
        self.wait_tx();
        self.channel.write_tx(word);
        self.wait_tx();
        let status = self.channel.regs().psr_iicmode().read();
        if status.nack().bit_is_set() {
            Err(ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown))
        } else if status.err().bit_is_set() {
            Err(ErrorKind::Bus)
        } else {
            Ok(())
        }
    }

    fn start(&mut self, address: u8, read: bool, repeated: bool) -> Result<(), ErrorKind> {
        let mut word = if repeated {
            MASTER_RESTART
        } else {
            MASTER_START
        };
        word |= u16::from(address) << 1;
        if read {
            word |= 1;
        }
        self.command(word)
    }

    fn read_byte(&mut self, ack: bool) -> Result<u8, ErrorKind> {
        self.command(if ack {
            MASTER_RECEIVE_ACK
        } else {
            MASTER_RECEIVE_NACK
        })?;
        while self
            .channel
            .regs()
            .psr_iicmode()
            .read()
            .rif()
            .bit_is_clear()
        {}
        let byte = self.channel.read_rx() as u8;
        self.channel.regs().pscr().write(|w| w.crif().set_bit());
        Ok(byte)
    }
}

impl<C, SCL, SDA> ErrorType for I2cMaster<C, SCL, SDA> {
    type Error = ErrorKind;
}

impl<C, SCL, SDA> I2c<SevenBitAddress> for I2cMaster<C, SCL, SDA>
where
    C: Deref<Target = pac::usic0_ch0::RegisterBlock>,
{
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        if address > 0x7f {
            return Err(ErrorKind::Other);
        }
        let mut first = true;
        let mut previous_read = false;
        for operation in operations.iter_mut() {
            let read = matches!(operation, Operation::Read(_));
            if first || read != previous_read {
                self.start(address, read, !first)?;
            }
            match operation {
                Operation::Write(bytes) => {
                    for byte in *bytes {
                        self.command(MASTER_SEND | u16::from(*byte))?;
                    }
                }
                Operation::Read(bytes) => {
                    let last = bytes.len().saturating_sub(1);
                    for (index, byte) in bytes.iter_mut().enumerate() {
                        *byte = self.read_byte(index != last)?;
                    }
                }
            }
            first = false;
            previous_read = read;
        }
        self.command(MASTER_STOP)
    }
}
