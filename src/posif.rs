//! Position interface: quadrature decoding, hall sensing, and multi-channel patterns.
//!
//! XMC4300 has no POSIF, so this module is not compiled for that device.

use core::ops::Deref;

use crate::pac::{self, POSIF0};
#[cfg(not(feature = "xmc4500"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
use crate::pac::POSIF1;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Hall,
    Quadrature,
    MultiChannel,
    QuadratureAndMultiChannel,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Event {
    HallCorrect,
    HallWrong,
    HallIdle,
    Error,
    Count,
    Direction,
}

pub trait PosifExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Posif<Self>
    where
        Self: Sized;
}

pub struct Posif<P> {
    regs: P,
}

impl Sealed for POSIF0 {}

impl PosifExt for POSIF0 {
    fn constrain(self, scu: &mut Scu) -> Posif<Self> {
        let posif = Posif { regs: self };
        posif.enable(scu);
        posif
    }
}

impl Posif<POSIF0> {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(not(feature = "xmc4500"))]
        scu.ungate_peripheral_clock(PeripheralClock::Posif0);
        scu.deassert_peripheral_reset(PeripheralReset::Posif0);
    }

    pub fn disable(&self, scu: &mut Scu) {
        scu.assert_peripheral_reset(PeripheralReset::Posif0);
        #[cfg(not(feature = "xmc4500"))]
        scu.gate_peripheral_clock(PeripheralClock::Posif0);
    }
}

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
impl Sealed for POSIF1 {}

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
impl PosifExt for POSIF1 {
    fn constrain(self, scu: &mut Scu) -> Posif<Self> {
        let posif = Posif { regs: self };
        posif.enable(scu);
        posif
    }
}

#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
impl Posif<POSIF1> {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(not(feature = "xmc4500"))]
        scu.ungate_peripheral_clock(PeripheralClock::Posif1);
        scu.deassert_peripheral_reset(PeripheralReset::Posif1);
    }

    pub fn disable(&self, scu: &mut Scu) {
        scu.assert_peripheral_reset(PeripheralReset::Posif1);
        #[cfg(not(feature = "xmc4500"))]
        scu.gate_peripheral_clock(PeripheralClock::Posif1);
    }
}

impl<P> Posif<P>
where
    P: Deref<Target = pac::posif0::RegisterBlock>,
{
    pub fn set_mode(&self, mode: Mode) {
        self.regs.pconf().modify(|_, w| {
            match mode {
                Mode::Hall => w.fsel().value1(),
                Mode::Quadrature => w.fsel().value2(),
                Mode::MultiChannel => w.fsel().value3(),
                Mode::QuadratureAndMultiChannel => w.fsel().value4(),
            };
            w
        });
    }

    pub fn start(&self) {
        self.regs.pruns().write(|w| w.srb().set_bit());
    }

    pub fn stop(&self) {
        self.regs.prunc().write(|w| w.crb().set_bit());
    }

    /// Last sampled hall pattern, `0..=7`.
    pub fn hall_pattern(&self) -> u8 {
        self.regs.halp().read().hcp().bits()
    }

    pub fn expected_hall_pattern(&self) -> u8 {
        self.regs.halp().read().hep().bits()
    }

    /// Programs the hall shadow patterns. Each value is `0..=7`.
    pub fn set_hall_patterns(&self, current: u8, expected: u8) -> Result<(), u8> {
        if current > 7 || expected > 7 {
            return Err(current);
        }
        self.regs.halps().write(|w| unsafe {
            w.hcps().bits(current);
            w.heps().bits(expected);
            w
        });
        self.regs.mcms().write(|w| w.sthr().set_bit());
        Ok(())
    }

    pub fn multi_channel_pattern(&self) -> u16 {
        self.regs.mcm().read().mcmp().bits()
    }

    pub fn set_multi_channel_pattern(&self, pattern: u16) {
        self.regs
            .mcsm()
            .write(|w| unsafe { w.mcmps().bits(pattern) });
        self.regs.mcms().write(|w| w.stmr().set_bit());
    }

    /// Quadrature count direction. `true` is the positive direction.
    pub fn quadrature_direction(&self) -> bool {
        self.regs.qdc().read().dval().bit()
    }

    pub fn listen(&self, event: Event) {
        self.regs.pflge().modify(|_, w| {
            match event {
                Event::HallCorrect => w.eche().set_bit(),
                Event::HallWrong => w.ewhe().set_bit(),
                Event::HallIdle => w.ehie().set_bit(),
                Event::Error => w.eerr().set_bit(),
                Event::Count => w.ecnt().set_bit(),
                Event::Direction => w.edir().set_bit(),
            };
            w
        });
    }

    pub fn unlisten(&self, event: Event) {
        self.regs.pflge().modify(|_, w| {
            match event {
                Event::HallCorrect => w.eche().clear_bit(),
                Event::HallWrong => w.ewhe().clear_bit(),
                Event::HallIdle => w.ehie().clear_bit(),
                Event::Error => w.eerr().clear_bit(),
                Event::Count => w.ecnt().clear_bit(),
                Event::Direction => w.edir().clear_bit(),
            };
            w
        });
    }

    pub fn is_event_pending(&self, event: Event) -> bool {
        let flags = self.regs.pflg().read();
        match event {
            Event::HallCorrect => flags.ches().bit_is_set(),
            Event::HallWrong => flags.whes().bit_is_set(),
            Event::HallIdle => flags.hies().bit_is_set(),
            Event::Error => flags.errs().bit_is_set(),
            Event::Count => flags.cnts().bit_is_set(),
            Event::Direction => flags.dirs().bit_is_set(),
        }
    }

    pub fn clear_event(&self, event: Event) {
        self.regs.rpflg().write(|w| {
            match event {
                Event::HallCorrect => w.rche().set_bit(),
                Event::HallWrong => w.rwhe().set_bit(),
                Event::HallIdle => w.rhie().set_bit(),
                Event::Error => w.rerr().set_bit(),
                Event::Count => w.rcnt().set_bit(),
                Event::Direction => w.rdir().set_bit(),
            };
            w
        });
    }
}
