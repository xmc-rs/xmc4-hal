//! CCU8 capture/compare modules.
//!
//! Same slice model as CCU4, plus a second compare channel and dead time on
//! each output.

use core::ops::Deref;

use embedded_hal_02::PwmPin;

use crate::pac::{self, CCU80};
#[cfg(not(feature = "xmc4500"))]
use crate::scu::PeripheralClock;
use crate::scu::{Clock, PeripheralReset, Scu};
use crate::sealed::Sealed;
use crate::time::Hertz;

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
use crate::pac::CCU81;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Event {
    PeriodMatch,
    OneMatch,
    Compare1Up,
    Compare1Down,
    Compare2Up,
    Compare2Down,
    Event0,
    Event1,
    Event2,
}

pub trait Ccu80Ext: Sealed {
    fn constrain(
        self,
        scu: &mut Scu,
        slices: (
            pac::CCU80_CC80,
            pac::CCU80_CC81,
            pac::CCU80_CC82,
            pac::CCU80_CC83,
        ),
    ) -> Ccu8x<CCU80, pac::CCU80_CC80, pac::CCU80_CC81, pac::CCU80_CC82, pac::CCU80_CC83>;
}

impl Sealed for CCU80 {}

impl Ccu80Ext for CCU80 {
    fn constrain(
        self,
        scu: &mut Scu,
        slices: (
            pac::CCU80_CC80,
            pac::CCU80_CC81,
            pac::CCU80_CC82,
            pac::CCU80_CC83,
        ),
    ) -> Ccu8x<CCU80, pac::CCU80_CC80, pac::CCU80_CC81, pac::CCU80_CC82, pac::CCU80_CC83> {
        Ccu8x::new(self, slices, scu, PeripheralReset::Ccu80, clock_ccu80())
    }
}

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
pub trait Ccu81Ext: Sealed {
    fn constrain(
        self,
        scu: &mut Scu,
        slices: (
            pac::CCU81_CC80,
            pac::CCU81_CC81,
            pac::CCU81_CC82,
            pac::CCU81_CC83,
        ),
    ) -> Ccu8x<CCU81, pac::CCU81_CC80, pac::CCU81_CC81, pac::CCU81_CC82, pac::CCU81_CC83>;
}

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
impl Sealed for CCU81 {}

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
impl Ccu81Ext for CCU81 {
    fn constrain(
        self,
        scu: &mut Scu,
        slices: (
            pac::CCU81_CC80,
            pac::CCU81_CC81,
            pac::CCU81_CC82,
            pac::CCU81_CC83,
        ),
    ) -> Ccu8x<CCU81, pac::CCU81_CC80, pac::CCU81_CC81, pac::CCU81_CC82, pac::CCU81_CC83> {
        Ccu8x::new(self, slices, scu, PeripheralReset::Ccu81, clock_ccu81())
    }
}

#[cfg(not(feature = "xmc4500"))]
fn clock_ccu80() -> Option<PeripheralClock> {
    Some(PeripheralClock::Ccu80)
}
#[cfg(feature = "xmc4500")]
fn clock_ccu80() -> Option<PeripheralClock> {
    None
}
#[cfg(all(
    not(feature = "xmc4500"),
    any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800")
))]
fn clock_ccu81() -> Option<PeripheralClock> {
    Some(PeripheralClock::Ccu81)
}
#[cfg(feature = "xmc4500")]
fn clock_ccu81() -> Option<PeripheralClock> {
    None
}

#[cfg(feature = "xmc4500")]
enum PeripheralClock {}

pub struct Ccu8x<M, S0, S1, S2, S3> {
    module: M,
    s0: S0,
    s1: S1,
    s2: S2,
    s3: S3,
    reset: PeripheralReset,
    #[cfg_attr(feature = "xmc4500", allow(dead_code))]
    clock: Option<PeripheralClock>,
}

impl<M, S0, S1, S2, S3> Ccu8x<M, S0, S1, S2, S3>
where
    M: Deref<Target = pac::ccu80::RegisterBlock>,
    S0: Deref<Target = pac::ccu80_cc80::RegisterBlock>,
    S1: Deref<Target = pac::ccu80_cc80::RegisterBlock>,
    S2: Deref<Target = pac::ccu80_cc80::RegisterBlock>,
    S3: Deref<Target = pac::ccu80_cc80::RegisterBlock>,
{
    fn new(
        module: M,
        slices: (S0, S1, S2, S3),
        scu: &mut Scu,
        reset: PeripheralReset,
        clock: Option<PeripheralClock>,
    ) -> Self {
        let ccu = Self {
            module,
            s0: slices.0,
            s1: slices.1,
            s2: slices.2,
            s3: slices.3,
            reset,
            clock,
        };
        ccu.enable(scu);
        ccu
    }

    pub fn enable(&self, scu: &mut Scu) {
        scu.enable_clock(Clock::Ccu);
        #[cfg(not(feature = "xmc4500"))]
        if let Some(clock) = self.clock {
            scu.ungate_peripheral_clock(clock);
        }
        scu.deassert_peripheral_reset(self.reset);
    }

    pub fn disable(&self, scu: &mut Scu) {
        scu.assert_peripheral_reset(self.reset);
        #[cfg(not(feature = "xmc4500"))]
        if let Some(clock) = self.clock {
            scu.gate_peripheral_clock(clock);
        }
        scu.disable_clock(Clock::Ccu);
    }

    pub fn slice0(&self) -> Slice<'_> {
        Slice::new(&self.s0, &self.module, 0)
    }
    pub fn slice1(&self) -> Slice<'_> {
        Slice::new(&self.s1, &self.module, 1)
    }
    pub fn slice2(&self) -> Slice<'_> {
        Slice::new(&self.s2, &self.module, 2)
    }
    pub fn slice3(&self) -> Slice<'_> {
        Slice::new(&self.s3, &self.module, 3)
    }
}

pub struct Slice<'a> {
    slice: &'a pac::ccu80_cc80::RegisterBlock,
    module: &'a pac::ccu80::RegisterBlock,
    index: u8,
}

impl<'a> Slice<'a> {
    fn new(
        slice: &'a pac::ccu80_cc80::RegisterBlock,
        module: &'a pac::ccu80::RegisterBlock,
        index: u8,
    ) -> Self {
        Self {
            slice,
            module,
            index,
        }
    }

    pub fn init_compare(&self) {
        self.slice.tc().modify(|_, w| w.tcm().clear_bit());
        self.leave_idle();
    }

    pub fn init_capture(&self) {
        self.slice.tc().modify(|_, w| w.tcm().clear_bit());
        self.leave_idle();
    }

    pub fn set_prescaler(&self, div_pow2: u8) -> Result<(), u8> {
        if div_pow2 > 15 {
            return Err(div_pow2);
        }
        self.slice
            .psc()
            .write(|w| unsafe { w.psiv().bits(div_pow2) });
        Ok(())
    }

    pub fn set_frequency(&self, ccu_clk: Hertz, freq: Hertz) -> Result<(), Hertz> {
        if freq.0 == 0 || freq.0 > ccu_clk.0 {
            return Err(freq);
        }
        let mut ticks = ccu_clk.0 / freq.0;
        let mut prescaler = 0u8;
        while ticks > 65536 && prescaler < 15 {
            ticks /= 2;
            prescaler += 1;
        }
        if ticks == 0 || ticks > 65536 {
            return Err(freq);
        }
        self.set_prescaler(prescaler).ok();
        self.set_period((ticks - 1) as u16);
        Ok(())
    }

    pub fn set_period(&self, period: u16) {
        self.slice.prs().write(|w| unsafe { w.prs().bits(period) });
        self.transfer_period();
    }

    /// Compare value for channel 1 or 2.
    pub fn set_compare(&self, channel: u8, compare: u16) -> Result<(), u8> {
        match channel {
            1 => {
                self.slice
                    .cr1s()
                    .write(|w| unsafe { w.cr1s().bits(compare) });
                self.transfer_compare(1);
                Ok(())
            }
            2 => {
                self.slice
                    .cr2s()
                    .write(|w| unsafe { w.cr2s().bits(compare) });
                self.transfer_compare(2);
                Ok(())
            }
            _ => Err(channel),
        }
    }

    pub fn compare(&self, channel: u8) -> Option<u16> {
        match channel {
            1 => Some(self.slice.cr1s().read().cr1s().bits()),
            2 => Some(self.slice.cr2s().read().cr2s().bits()),
            _ => None,
        }
    }

    pub fn period(&self) -> u16 {
        self.slice.prs().read().prs().bits()
    }

    /// Rising and falling dead time, in timer ticks, for channel 1 or 2.
    pub fn set_dead_time(&self, channel: u8, rise: u8, fall: u8) -> Result<(), u8> {
        match channel {
            1 => {
                self.slice.dc1r().write(|w| unsafe {
                    w.dt1r().bits(rise);
                    w.dt1f().bits(fall);
                    w
                });
                self.slice.dtc().modify(|_, w| w.dte1().set_bit());
                Ok(())
            }
            2 => {
                self.slice.dc2r().write(|w| unsafe {
                    w.dt2r().bits(rise);
                    w.dt2f().bits(fall);
                    w
                });
                self.slice.dtc().modify(|_, w| w.dte2().set_bit());
                Ok(())
            }
            _ => Err(channel),
        }
    }

    pub fn disable_dead_time(&self, channel: u8) -> Result<(), u8> {
        match channel {
            1 => {
                self.slice.dtc().modify(|_, w| w.dte1().clear_bit());
                Ok(())
            }
            2 => {
                self.slice.dtc().modify(|_, w| w.dte2().clear_bit());
                Ok(())
            }
            _ => Err(channel),
        }
    }

    pub fn capture(&self, index: usize) -> Option<u16> {
        Some(match index {
            0 => self.slice.c0v().read().captv().bits(),
            1 => self.slice.c1v().read().captv().bits(),
            2 => self.slice.c2v().read().captv().bits(),
            3 => self.slice.c3v().read().captv().bits(),
            _ => return None,
        })
    }

    pub fn start(&self) {
        self.leave_idle();
        self.slice.tcset().write(|w| w.trbs().set_bit());
    }

    pub fn stop(&self) {
        self.slice.tcclr().write(|w| w.trbc().set_bit());
    }

    pub fn listen(&self, event: Event) {
        self.slice.inte().modify(|_, w| match event {
            Event::PeriodMatch => w.pme().set_bit(),
            Event::OneMatch => w.ome().set_bit(),
            Event::Compare1Up => w.cmu1e().set_bit(),
            Event::Compare1Down => w.cmd1e().set_bit(),
            Event::Compare2Up => w.cmu2e().set_bit(),
            Event::Compare2Down => w.cmd2e().set_bit(),
            Event::Event0 => w.e0ae().set_bit(),
            Event::Event1 => w.e1ae().set_bit(),
            Event::Event2 => w.e2ae().set_bit(),
        });
    }

    pub fn unlisten(&self, event: Event) {
        self.slice.inte().modify(|_, w| match event {
            Event::PeriodMatch => w.pme().clear_bit(),
            Event::OneMatch => w.ome().clear_bit(),
            Event::Compare1Up => w.cmu1e().clear_bit(),
            Event::Compare1Down => w.cmd1e().clear_bit(),
            Event::Compare2Up => w.cmu2e().clear_bit(),
            Event::Compare2Down => w.cmd2e().clear_bit(),
            Event::Event0 => w.e0ae().clear_bit(),
            Event::Event1 => w.e1ae().clear_bit(),
            Event::Event2 => w.e2ae().clear_bit(),
        });
    }

    pub fn is_event_pending(&self, event: Event) -> bool {
        let ints = self.slice.ints().read();
        match event {
            Event::PeriodMatch => ints.pmus().bit_is_set(),
            Event::OneMatch => ints.omds().bit_is_set(),
            Event::Compare1Up => ints.cmu1s().bit_is_set(),
            Event::Compare1Down => ints.cmd1s().bit_is_set(),
            Event::Compare2Up => ints.cmu2s().bit_is_set(),
            Event::Compare2Down => ints.cmd2s().bit_is_set(),
            Event::Event0 => ints.e0as().bit_is_set(),
            Event::Event1 => ints.e1as().bit_is_set(),
            Event::Event2 => ints.e2as().bit_is_set(),
        }
    }

    pub fn clear_event(&self, event: Event) {
        self.slice.swr().write(|w| match event {
            Event::PeriodMatch => w.rpm().set_bit(),
            Event::OneMatch => w.rom().set_bit(),
            Event::Compare1Up => w.rcm1u().set_bit(),
            Event::Compare1Down => w.rcm1d().set_bit(),
            Event::Compare2Up => w.rcm2u().set_bit(),
            Event::Compare2Down => w.rcm2d().set_bit(),
            Event::Event0 => w.re0a().set_bit(),
            Event::Event1 => w.re1a().set_bit(),
            Event::Event2 => w.re2a().set_bit(),
        });
    }

    fn leave_idle(&self) {
        self.module.gidlc().write(|w| match self.index {
            0 => w.cs0i().set_bit(),
            1 => w.cs1i().set_bit(),
            2 => w.cs2i().set_bit(),
            _ => w.cs3i().set_bit(),
        });
    }

    fn transfer_period(&self) {
        self.module.gcss().write(|w| match self.index {
            0 => w.s0pse().set_bit(),
            1 => w.s1pse().set_bit(),
            2 => w.s2pse().set_bit(),
            _ => w.s3pse().set_bit(),
        });
    }

    fn transfer_compare(&self, channel: u8) {
        self.module.gcss().write(|w| match (self.index, channel) {
            (0, 1) => w.s0st1s().set_bit(),
            (0, _) => w.s0st2s().set_bit(),
            (1, 1) => w.s1st1s().set_bit(),
            (1, _) => w.s1st2s().set_bit(),
            (2, 1) => w.s2st1s().set_bit(),
            (2, _) => w.s2st2s().set_bit(),
            (_, 1) => w.s3st1s().set_bit(),
            (_, _) => w.s3st2s().set_bit(),
        });
    }
}

impl<'a> PwmPin for Slice<'a> {
    type Duty = u16;

    fn disable(&mut self) {
        self.stop();
    }

    fn enable(&mut self) {
        self.init_compare();
        self.start();
    }

    fn get_duty(&self) -> Self::Duty {
        self.compare(1).unwrap_or(0)
    }

    fn get_max_duty(&self) -> Self::Duty {
        self.period()
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        let _ = self.set_compare(1, duty);
    }
}
