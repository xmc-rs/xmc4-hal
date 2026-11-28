//! DMA line router overrun status.
//!
//! There is no transfer driver for this block. The router reports which DMA
//! request line overran, and that flag is cleared here.

use crate::pac::DLR;
use crate::sealed::Sealed;

pub trait DlrExt: Sealed {
    fn constrain(self) -> Dlr;
}

impl Sealed for DLR {}

impl DlrExt for DLR {
    fn constrain(self) -> Dlr {
        Dlr { regs: self }
    }
}

pub struct Dlr {
    regs: DLR,
}

impl Dlr {
    /// Highest request line this device routes.
    pub const LINES: u8 = {
        #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
        {
            12
        }
        #[cfg(not(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800")))]
        {
            8
        }
    };

    pub fn overrun(&self, line: u8) -> Result<bool, u8> {
        if line >= Self::LINES {
            return Err(line);
        }
        let status = self.regs.ovrstat().read();
        Ok(match line {
            0 => status.ln0().bit_is_set(),
            1 => status.ln1().bit_is_set(),
            2 => status.ln2().bit_is_set(),
            3 => status.ln3().bit_is_set(),
            4 => status.ln4().bit_is_set(),
            5 => status.ln5().bit_is_set(),
            6 => status.ln6().bit_is_set(),
            7 => status.ln7().bit_is_set(),
            #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
            8 => status.ln8().bit_is_set(),
            #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
            9 => status.ln9().bit_is_set(),
            #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
            10 => status.ln10().bit_is_set(),
            #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
            11 => status.ln11().bit_is_set(),
            _ => return Err(line),
        })
    }

    pub fn clear_overrun(&self, line: u8) -> Result<(), u8> {
        if line >= Self::LINES {
            return Err(line);
        }
        self.regs.ovrclr().write(|w| {
            match line {
                0 => {
                    w.ln0().set_bit();
                }
                1 => {
                    w.ln1().set_bit();
                }
                2 => {
                    w.ln2().set_bit();
                }
                3 => {
                    w.ln3().set_bit();
                }
                4 => {
                    w.ln4().set_bit();
                }
                5 => {
                    w.ln5().set_bit();
                }
                6 => {
                    w.ln6().set_bit();
                }
                7 => {
                    w.ln7().set_bit();
                }
                #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
                8 => {
                    w.ln8().set_bit();
                }
                #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
                9 => {
                    w.ln9().set_bit();
                }
                #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
                10 => {
                    w.ln10().set_bit();
                }
                #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
                11 => {
                    w.ln11().set_bit();
                }
                _ => {}
            };
            w
        });
        Ok(())
    }
}
