//! LED and touch-sense controller.
//!
//! Global enable, LED column setup, touch-sense input setup, and the pad
//! values latched in the line registers.

use crate::pac::LEDTS0;
#[cfg(not(feature = "xmc4500"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Event {
    TimeSlice,
    TimeFrame,
    TimePeriod,
}

pub trait LedtsExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Ledts;
}

impl Sealed for LEDTS0 {}

impl LedtsExt for LEDTS0 {
    fn constrain(self, scu: &mut Scu) -> Ledts {
        let ledts = Ledts { regs: self };
        ledts.enable(scu);
        ledts
    }
}

pub struct Ledts {
    regs: LEDTS0,
}

impl Ledts {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(not(feature = "xmc4500"))]
        scu.ungate_peripheral_clock(PeripheralClock::Ledtscu0);
        scu.deassert_peripheral_reset(PeripheralReset::Ledts0);
    }

    pub fn disable(&self, scu: &mut Scu) {
        scu.assert_peripheral_reset(PeripheralReset::Ledts0);
        #[cfg(not(feature = "xmc4500"))]
        scu.gate_peripheral_clock(PeripheralClock::Ledtscu0);
    }

    pub fn set_prescaler(&self, divider: u16) {
        self.regs
            .globctl()
            .modify(|_, w| unsafe { w.clk_ps().bits(divider) });
    }

    pub fn enable_led(&self) {
        self.regs.globctl().modify(|_, w| w.ld_en().set_bit());
    }

    pub fn disable_led(&self) {
        self.regs.globctl().modify(|_, w| w.ld_en().clear_bit());
    }

    pub fn enable_touch(&self) {
        self.regs.globctl().modify(|_, w| w.ts_en().set_bit());
    }

    pub fn disable_touch(&self) {
        self.regs.globctl().modify(|_, w| w.ts_en().clear_bit());
    }

    /// Number of LED columns, `1..=8` stored as count-1.
    pub fn set_led_columns(&self, columns: u8) -> Result<(), u8> {
        if !(1..=8).contains(&columns) {
            return Err(columns);
        }
        self.regs
            .fnctl()
            .modify(|_, w| unsafe { w.nr_ledcol().bits(columns - 1) });
        Ok(())
    }

    /// Number of touch-sense inputs, `1..=8` stored as count-1.
    pub fn set_touch_inputs(&self, inputs: u8) -> Result<(), u8> {
        if !(1..=8).contains(&inputs) {
            return Err(inputs);
        }
        self.regs
            .fnctl()
            .modify(|_, w| unsafe { w.nr_tsin().bits(inputs - 1) });
        Ok(())
    }

    /// Active pad, `0..=7`.
    pub fn set_pad(&self, pad: u8) -> Result<(), u8> {
        if pad > 7 {
            return Err(pad);
        }
        self.regs
            .fnctl()
            .modify(|_, w| unsafe { w.padt().bits(pad) });
        Ok(())
    }

    /// LED pattern for one line register. `line` is 0 or 1 and `pattern` is four lane values.
    pub fn set_line(&self, line: u8, pattern: [u8; 4]) -> Result<(), u8> {
        match line {
            0 => {
                self.regs.line0().modify(|_, w| unsafe {
                    w.line_0().bits(pattern[0]);
                    w.line_1().bits(pattern[1]);
                    w.line_2().bits(pattern[2]);
                    w.line_3().bits(pattern[3]);
                    w
                });
            }
            1 => {
                self.regs.line1().modify(|_, w| unsafe {
                    w.line_4().bits(pattern[0]);
                    w.line_5().bits(pattern[1]);
                    w.line_6().bits(pattern[2]);
                    w.line_a().bits(pattern[3]);
                    w
                });
            }
            _ => return Err(line),
        }
        Ok(())
    }

    pub fn pad(&self, line: u8) -> Result<[u8; 4], u8> {
        match line {
            0 => {
                let read = self.regs.line0().read();
                Ok([
                    read.line_0().bits(),
                    read.line_1().bits(),
                    read.line_2().bits(),
                    read.line_3().bits(),
                ])
            }
            1 => {
                let read = self.regs.line1().read();
                Ok([
                    read.line_4().bits(),
                    read.line_5().bits(),
                    read.line_6().bits(),
                    read.line_a().bits(),
                ])
            }
            _ => Err(line),
        }
    }

    pub fn touch_counter(&self) -> u16 {
        self.regs.tsval().read().tsctrval().bits()
    }

    pub fn listen(&self, event: Event) {
        self.regs.globctl().modify(|_, w| {
            match event {
                Event::TimeSlice => w.its_en().set_bit(),
                Event::TimeFrame => w.itf_en().set_bit(),
                Event::TimePeriod => w.itp_en().set_bit(),
            };
            w
        });
    }

    pub fn unlisten(&self, event: Event) {
        self.regs.globctl().modify(|_, w| {
            match event {
                Event::TimeSlice => w.its_en().clear_bit(),
                Event::TimeFrame => w.itf_en().clear_bit(),
                Event::TimePeriod => w.itp_en().clear_bit(),
            };
            w
        });
    }

    pub fn is_event_pending(&self, event: Event) -> bool {
        let flags = self.regs.evfr().read();
        match event {
            Event::TimeSlice => flags.tsf().bit_is_set(),
            Event::TimeFrame => flags.tff().bit_is_set(),
            Event::TimePeriod => flags.tpf().bit_is_set(),
        }
    }

    pub fn clear_event(&self, event: Event) {
        self.regs.evfr().modify(|_, w| {
            match event {
                Event::TimeSlice => w.ctsf().set_bit(),
                Event::TimeFrame => w.ctff().set_bit(),
                Event::TimePeriod => w.ctpf().set_bit(),
            };
            w
        });
    }
}
