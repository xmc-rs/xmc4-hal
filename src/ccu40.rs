//! CCU4 capture/compare modules.
//!
//! Each module owns its four slices. A slice in compare mode implements
//! [`embedded_hal_02::PwmPin`]. Period and compare values are written to the
//! shadow registers and transferred with `GCSS`.

use core::ops::Deref;

use embedded_hal_02::PwmPin;

use crate::pac::{self, CCU40, CCU41};
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
use crate::pac::{CCU42, CCU43};

/// Slice event reported in `INTS` and enabled in `INTE`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Event {
    PeriodMatch,
    OneMatch,
    CompareMatchUp,
    CompareMatchDown,
    Event0,
    Event1,
    Event2,
}

pub trait Ccu40Ext: Sealed {
    fn constrain(
        self,
        scu: &mut Scu,
        slices: (
            pac::CCU40_CC40,
            pac::CCU40_CC41,
            pac::CCU40_CC42,
            pac::CCU40_CC43,
        ),
    ) -> Ccu4x<CCU40, pac::CCU40_CC40, pac::CCU40_CC41, pac::CCU40_CC42, pac::CCU40_CC43>;
}

impl Sealed for CCU40 {}

impl Ccu40Ext for CCU40 {
    fn constrain(
        self,
        scu: &mut Scu,
        slices: (
            pac::CCU40_CC40,
            pac::CCU40_CC41,
            pac::CCU40_CC42,
            pac::CCU40_CC43,
        ),
    ) -> Ccu4x<CCU40, pac::CCU40_CC40, pac::CCU40_CC41, pac::CCU40_CC42, pac::CCU40_CC43> {
        Ccu4x::new(self, slices, scu, PeripheralReset::Ccu40, clock_ccu40())
    }
}

pub trait Ccu41Ext: Sealed {
    fn constrain(
        self,
        scu: &mut Scu,
        slices: (
            pac::CCU41_CC40,
            pac::CCU41_CC41,
            pac::CCU41_CC42,
            pac::CCU41_CC43,
        ),
    ) -> Ccu4x<CCU41, pac::CCU41_CC40, pac::CCU41_CC41, pac::CCU41_CC42, pac::CCU41_CC43>;
}

impl Sealed for CCU41 {}

impl Ccu41Ext for CCU41 {
    fn constrain(
        self,
        scu: &mut Scu,
        slices: (
            pac::CCU41_CC40,
            pac::CCU41_CC41,
            pac::CCU41_CC42,
            pac::CCU41_CC43,
        ),
    ) -> Ccu4x<CCU41, pac::CCU41_CC40, pac::CCU41_CC41, pac::CCU41_CC42, pac::CCU41_CC43> {
        Ccu4x::new(self, slices, scu, PeripheralReset::Ccu41, clock_ccu41())
    }
}

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
pub trait Ccu42Ext: Sealed {
    fn constrain(
        self,
        scu: &mut Scu,
        slices: (
            pac::CCU42_CC40,
            pac::CCU42_CC41,
            pac::CCU42_CC42,
            pac::CCU42_CC43,
        ),
    ) -> Ccu4x<CCU42, pac::CCU42_CC40, pac::CCU42_CC41, pac::CCU42_CC42, pac::CCU42_CC43>;
}

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
impl Sealed for CCU42 {}

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
impl Ccu42Ext for CCU42 {
    fn constrain(
        self,
        scu: &mut Scu,
        slices: (
            pac::CCU42_CC40,
            pac::CCU42_CC41,
            pac::CCU42_CC42,
            pac::CCU42_CC43,
        ),
    ) -> Ccu4x<CCU42, pac::CCU42_CC40, pac::CCU42_CC41, pac::CCU42_CC42, pac::CCU42_CC43> {
        Ccu4x::new(self, slices, scu, PeripheralReset::Ccu42, clock_ccu42())
    }
}

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
pub trait Ccu43Ext: Sealed {
    fn constrain(
        self,
        scu: &mut Scu,
        slices: (
            pac::CCU43_CC40,
            pac::CCU43_CC41,
            pac::CCU43_CC42,
            pac::CCU43_CC43,
        ),
    ) -> Ccu4x<CCU43, pac::CCU43_CC40, pac::CCU43_CC41, pac::CCU43_CC42, pac::CCU43_CC43>;
}

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
impl Sealed for CCU43 {}

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
impl Ccu43Ext for CCU43 {
    fn constrain(
        self,
        scu: &mut Scu,
        slices: (
            pac::CCU43_CC40,
            pac::CCU43_CC41,
            pac::CCU43_CC42,
            pac::CCU43_CC43,
        ),
    ) -> Ccu4x<CCU43, pac::CCU43_CC40, pac::CCU43_CC41, pac::CCU43_CC42, pac::CCU43_CC43> {
        Ccu4x::new(self, slices, scu, PeripheralReset::Ccu43, clock_ccu43())
    }
}

#[cfg(not(feature = "xmc4500"))]
fn clock_ccu40() -> Option<PeripheralClock> {
    Some(PeripheralClock::Ccu40)
}
#[cfg(feature = "xmc4500")]
fn clock_ccu40() -> Option<PeripheralClock> {
    None
}
#[cfg(not(feature = "xmc4500"))]
fn clock_ccu41() -> Option<PeripheralClock> {
    Some(PeripheralClock::Ccu41)
}
#[cfg(feature = "xmc4500")]
fn clock_ccu41() -> Option<PeripheralClock> {
    None
}

#[cfg(all(
    not(feature = "xmc4500"),
    any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800")
))]
fn clock_ccu42() -> Option<PeripheralClock> {
    Some(PeripheralClock::Ccu42)
}
#[cfg(feature = "xmc4500")]
fn clock_ccu42() -> Option<PeripheralClock> {
    None
}

#[cfg(all(
    not(feature = "xmc4500"),
    any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800")
))]
fn clock_ccu43() -> Option<PeripheralClock> {
    Some(PeripheralClock::Ccu43)
}
#[cfg(feature = "xmc4500")]
fn clock_ccu43() -> Option<PeripheralClock> {
    None
}

/// On XMC4500 the clock-gate block does not exist, so the option is empty.
#[cfg(feature = "xmc4500")]
enum PeripheralClock {}

/// One CCU4 module and the four slices that belong to it.
pub struct Ccu4x<M, S0, S1, S2, S3> {
    module: M,
    s0: S0,
    s1: S1,
    s2: S2,
    s3: S3,
    reset: PeripheralReset,
    #[cfg_attr(feature = "xmc4500", allow(dead_code))]
    clock: Option<PeripheralClock>,
}

impl<M, S0, S1, S2, S3> Ccu4x<M, S0, S1, S2, S3>
where
    M: Deref<Target = pac::ccu40::RegisterBlock>,
    S0: Deref<Target = pac::ccu40_cc40::RegisterBlock>,
    S1: Deref<Target = pac::ccu40_cc40::RegisterBlock>,
    S2: Deref<Target = pac::ccu40_cc40::RegisterBlock>,
    S3: Deref<Target = pac::ccu40_cc40::RegisterBlock>,
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
        Slice {
            slice: &self.s0,
            module: &self.module,
            index: 0,
        }
    }
    pub fn slice1(&self) -> Slice<'_> {
        Slice {
            slice: &self.s1,
            module: &self.module,
            index: 1,
        }
    }
    pub fn slice2(&self) -> Slice<'_> {
        Slice {
            slice: &self.s2,
            module: &self.module,
            index: 2,
        }
    }
    pub fn slice3(&self) -> Slice<'_> {
        Slice {
            slice: &self.s3,
            module: &self.module,
            index: 3,
        }
    }
}

/// One compare/capture slice.
pub struct Slice<'a> {
    slice: &'a pac::ccu40_cc40::RegisterBlock,
    module: &'a pac::ccu40::RegisterBlock,
    index: u8,
}

impl<'a> Slice<'a> {
    /// Edge-aligned compare. The timer counts up to the period shadow value.
    pub fn init_compare(&self) {
        self.slice.tc().modify(|_, w| w.tcm().clear_bit());
        self.leave_idle();
    }

    /// Capture mode uses the same timer. Read the FIFO with [`capture`](Self::capture).
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

    /// Choose a prescaler so `freq` fits in the 16-bit period register.
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

    pub fn set_compare(&self, compare: u16) {
        self.slice.crs().write(|w| unsafe { w.crs().bits(compare) });
        self.transfer_compare();
    }

    pub fn period(&self) -> u16 {
        self.slice.prs().read().prs().bits()
    }

    pub fn compare(&self) -> u16 {
        self.slice.crs().read().crs().bits()
    }

    /// Most recent value in capture register `index` (0..=3).
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
            Event::CompareMatchUp => w.cmue().set_bit(),
            Event::CompareMatchDown => w.cmde().set_bit(),
            Event::Event0 => w.e0ae().set_bit(),
            Event::Event1 => w.e1ae().set_bit(),
            Event::Event2 => w.e2ae().set_bit(),
        });
    }

    pub fn unlisten(&self, event: Event) {
        self.slice.inte().modify(|_, w| match event {
            Event::PeriodMatch => w.pme().clear_bit(),
            Event::OneMatch => w.ome().clear_bit(),
            Event::CompareMatchUp => w.cmue().clear_bit(),
            Event::CompareMatchDown => w.cmde().clear_bit(),
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
            Event::CompareMatchUp => ints.cmus().bit_is_set(),
            Event::CompareMatchDown => ints.cmds().bit_is_set(),
            Event::Event0 => ints.e0as().bit_is_set(),
            Event::Event1 => ints.e1as().bit_is_set(),
            Event::Event2 => ints.e2as().bit_is_set(),
        }
    }

    pub fn clear_event(&self, event: Event) {
        self.slice.swr().write(|w| match event {
            Event::PeriodMatch => w.rpm().set_bit(),
            Event::OneMatch => w.rom().set_bit(),
            Event::CompareMatchUp => w.rcmu().set_bit(),
            Event::CompareMatchDown => w.rcmd().set_bit(),
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

    fn transfer_compare(&self) {
        self.module.gcss().write(|w| match self.index {
            0 => w.s0se().set_bit(),
            1 => w.s1se().set_bit(),
            2 => w.s2se().set_bit(),
            _ => w.s3se().set_bit(),
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
        self.compare()
    }

    fn get_max_duty(&self) -> Self::Duty {
        self.period()
    }

    fn set_duty(&mut self, duty: Self::Duty) {
        self.set_compare(duty);
    }
}
