#![allow(dead_code)]

use core::ops::Deref;

use embedded_hal_02::adc::{Channel, OneShot};

use crate::pac::{self, VADC, VADC_G0, VADC_G1};
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
use crate::pac::{VADC_G2, VADC_G3};

pub trait VadcExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Vadc;
}

impl Sealed for VADC {}

impl VadcExt for VADC {
    fn constrain(self, scu: &mut Scu) -> Vadc {
        let vadc = Vadc { regs: self };
        vadc.enable(scu);
        vadc.enable_module_clock();
        vadc
    }
}

pub struct Vadc {
    regs: VADC,
}

/// Types of service requests. Each group can raise up to 4 service requests and all groups together can have 4 service
/// requests.
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum ServiceRequest {
    GroupSR0,
    GroupSR1,
    GroupSR2,
    GroupSR3,
    SharedSR0,
    SharedSR1,
    SharedSR2,
    SharedSR3,
}

impl From<ServiceRequest> for u8 {
    fn from(bits: ServiceRequest) -> Self {
        match bits {
            ServiceRequest::GroupSR0 => 0,
            ServiceRequest::GroupSR1 => 1,
            ServiceRequest::GroupSR2 => 2,
            ServiceRequest::GroupSR3 => 3,
            ServiceRequest::SharedSR0 => 4,
            ServiceRequest::SharedSR1 => 5,
            ServiceRequest::SharedSR2 => 6,
            ServiceRequest::SharedSR3 => 7,
        }
    }
}

impl TryFrom<u8> for ServiceRequest {
    type Error = u8;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        match bits {
            0 => Ok(ServiceRequest::GroupSR0),
            1 => Ok(ServiceRequest::GroupSR1),
            2 => Ok(ServiceRequest::GroupSR2),
            3 => Ok(ServiceRequest::GroupSR3),
            4 => Ok(ServiceRequest::SharedSR0),
            5 => Ok(ServiceRequest::SharedSR1),
            6 => Ok(ServiceRequest::SharedSR2),
            7 => Ok(ServiceRequest::SharedSR3),
            other => Err(other),
        }
    }
}

/// Operational mode of a channel when a conversion is interrupted.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StartMode {
    /// Conversion completes without interruption
    WithoutInterruption,
    /// Conversion can be interrupted and then resumed.
    InterruptionResume,
    /// Conversion can be interrupted and not resume.
    InterruptionStop,
}

impl From<StartMode> for u8 {
    fn from(bits: StartMode) -> Self {
        match bits {
            StartMode::WithoutInterruption => 0,
            StartMode::InterruptionResume => 1,
            StartMode::InterruptionStop => 2,
        }
    }
}

impl TryFrom<u8> for StartMode {
    type Error = u8;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        match bits {
            0 => Ok(StartMode::WithoutInterruption),
            1 => Ok(StartMode::InterruptionResume),
            2 => Ok(StartMode::InterruptionStop),
            other => Err(other),
        }
    }
}

/// Types of edges that can start conversion based on an external source.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TriggerEdge {
    /// No external triggers
    None,
    /// Conversion starts on a falling edge of external source
    Falling,
    /// Conversion starts on rising edge of external source
    Rising,
    /// Falling and rising edge of external source can start conversion
    Any,
}

impl From<TriggerEdge> for u8 {
    fn from(bits: TriggerEdge) -> Self {
        match bits {
            TriggerEdge::None => 0,
            TriggerEdge::Falling => 1,
            TriggerEdge::Rising => 2,
            TriggerEdge::Any => 3,
        }
    }
}

impl TryFrom<u8> for TriggerEdge {
    type Error = u8;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        match bits {
            0 => Ok(TriggerEdge::None),
            1 => Ok(TriggerEdge::Falling),
            2 => Ok(TriggerEdge::Rising),
            3 => Ok(TriggerEdge::Any),
            other => Err(other),
        }
    }
}

/// Different possibilities that can act as an external input for triggering conversion. The results of the chosen item
/// differ per channel.
#[derive(Debug, Clone, Copy)]
pub enum TriggerInputSelect {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
}

impl From<TriggerInputSelect> for u8 {
    fn from(bits: TriggerInputSelect) -> Self {
        match bits {
            TriggerInputSelect::A => 0,
            TriggerInputSelect::B => 1,
            TriggerInputSelect::C => 2,
            TriggerInputSelect::D => 3,
            TriggerInputSelect::E => 4,
            TriggerInputSelect::F => 5,
            TriggerInputSelect::G => 6,
            TriggerInputSelect::H => 7,
            TriggerInputSelect::I => 8,
            TriggerInputSelect::J => 9,
            TriggerInputSelect::K => 10,
            TriggerInputSelect::L => 11,
            TriggerInputSelect::M => 12,
            TriggerInputSelect::N => 13,
            TriggerInputSelect::O => 14,
            TriggerInputSelect::P => 15,
        }
    }
}

impl TryFrom<u8> for TriggerInputSelect {
    type Error = u8;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        match bits {
            0 => Ok(TriggerInputSelect::A),
            1 => Ok(TriggerInputSelect::B),
            2 => Ok(TriggerInputSelect::C),
            3 => Ok(TriggerInputSelect::D),
            4 => Ok(TriggerInputSelect::E),
            5 => Ok(TriggerInputSelect::F),
            6 => Ok(TriggerInputSelect::G),
            7 => Ok(TriggerInputSelect::H),
            8 => Ok(TriggerInputSelect::I),
            9 => Ok(TriggerInputSelect::J),
            10 => Ok(TriggerInputSelect::K),
            11 => Ok(TriggerInputSelect::L),
            12 => Ok(TriggerInputSelect::M),
            13 => Ok(TriggerInputSelect::N),
            14 => Ok(TriggerInputSelect::O),
            15 => Ok(TriggerInputSelect::P),
            other => Err(other),
        }
    }
}

/// Different gating input possibilities that can gate conversion requests.
#[derive(Debug, Clone, Copy)]
pub enum GateInputSelect {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
}

impl From<GateInputSelect> for u8 {
    fn from(bits: GateInputSelect) -> Self {
        match bits {
            GateInputSelect::A => 0,
            GateInputSelect::B => 1,
            GateInputSelect::C => 2,
            GateInputSelect::D => 3,
            GateInputSelect::E => 4,
            GateInputSelect::F => 5,
            GateInputSelect::G => 6,
            GateInputSelect::H => 7,
            GateInputSelect::I => 8,
            GateInputSelect::J => 9,
            GateInputSelect::K => 10,
            GateInputSelect::L => 11,
            GateInputSelect::M => 12,
            GateInputSelect::N => 13,
            GateInputSelect::O => 14,
            GateInputSelect::P => 15,
        }
    }
}

impl TryFrom<u8> for GateInputSelect {
    type Error = u8;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        match bits {
            0 => Ok(GateInputSelect::A),
            1 => Ok(GateInputSelect::B),
            2 => Ok(GateInputSelect::C),
            3 => Ok(GateInputSelect::D),
            4 => Ok(GateInputSelect::E),
            5 => Ok(GateInputSelect::F),
            6 => Ok(GateInputSelect::G),
            7 => Ok(GateInputSelect::H),
            8 => Ok(GateInputSelect::I),
            9 => Ok(GateInputSelect::J),
            10 => Ok(GateInputSelect::K),
            11 => Ok(GateInputSelect::L),
            12 => Ok(GateInputSelect::M),
            13 => Ok(GateInputSelect::N),
            14 => Ok(GateInputSelect::O),
            15 => Ok(GateInputSelect::P),
            other => Err(other),
        }
    }
}

/// Conditions for gating conversion requests.
#[derive(Debug, Clone, Copy)]
pub enum GateMode {
    /// No external triggers are passed
    Block,
    /// All external triggers are passed
    Ignore,
    /// Gate signals that are active high are passed
    ActiveHigh,
    /// Gate signals that are active low are passed
    ActiveLow,
}

impl From<GateMode> for u8 {
    fn from(bits: GateMode) -> Self {
        match bits {
            GateMode::Block => 0,
            GateMode::Ignore => 1,
            GateMode::ActiveHigh => 2,
            GateMode::ActiveLow => 3,
        }
    }
}

impl TryFrom<u8> for GateMode {
    type Error = u8;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        match bits {
            0 => Ok(GateMode::Block),
            1 => Ok(GateMode::Ignore),
            2 => Ok(GateMode::ActiveHigh),
            3 => Ok(GateMode::ActiveLow),
            other => Err(other),
        }
    }
}

impl Vadc {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(not(feature = "xmc4500"))]
        scu.ungate_peripheral_clock(PeripheralClock::Vadc);
        scu.deassert_peripheral_reset(PeripheralReset::Vadc);
    }

    pub fn disable(&self, scu: &mut Scu) {
        scu.assert_peripheral_reset(PeripheralReset::Vadc);
        #[cfg(not(feature = "xmc4500"))]
        scu.gate_peripheral_clock(PeripheralClock::Vadc);
    }

    /// `CLC.DISR` clear turns the module clock on.
    pub fn enable_module_clock(&self) {
        self.regs.clc().modify(|_, w| w.disr().clear_bit());
    }

    pub fn disable_module_clock(&self) {
        self.regs.clc().modify(|_, w| w.disr().set_bit());
    }

    pub fn enable_sleep_mode(&self) {
        self.regs.clc().modify(|_, w| w.edis().clear_bit());
    }

    pub fn disable_sleep_mode(&self) {
        self.regs.clc().modify(|_, w| w.edis().set_bit());
    }

    /// Analog clock divider. `divwc` is set so the new divider is accepted.
    pub fn set_divider(&self, diva: u8) -> Result<(), u8> {
        if diva > 31 {
            return Err(diva);
        }
        self.regs.globcfg().modify(|_, w| unsafe {
            w.diva().bits(diva);
            w.divwc().set_bit();
            w
        });
        Ok(())
    }

    pub fn enable_startup_calibration(&self) {
        self.regs.globcfg().modify(|_, w| w.sucal().set_bit());
    }

    pub fn disable_startup_calibration(&self) {
        self.regs.globcfg().modify(|_, w| w.sucal().clear_bit());
    }

    /// Select channels of one group for the background scan. `mask` bit n is channel n.
    pub fn select_background(&self, group: usize, mask: u8) {
        self.regs.brssel(group).modify(|_, w| {
            if mask & 1 != 0 {
                w.chselg0().set_bit();
            } else {
                w.chselg0().clear_bit();
            }
            if mask & 2 != 0 {
                w.chselg1().set_bit();
            } else {
                w.chselg1().clear_bit();
            }
            if mask & 4 != 0 {
                w.chselg2().set_bit();
            } else {
                w.chselg2().clear_bit();
            }
            if mask & 8 != 0 {
                w.chselg3().set_bit();
            } else {
                w.chselg3().clear_bit();
            }
            if mask & 16 != 0 {
                w.chselg4().set_bit();
            } else {
                w.chselg4().clear_bit();
            }
            if mask & 32 != 0 {
                w.chselg5().set_bit();
            } else {
                w.chselg5().clear_bit();
            }
            if mask & 64 != 0 {
                w.chselg6().set_bit();
            } else {
                w.chselg6().clear_bit();
            }
            if mask & 128 != 0 {
                w.chselg7().set_bit();
            } else {
                w.chselg7().clear_bit();
            }
            w
        });
    }
}

pub trait GroupExt: Sealed {
    fn constrain(self) -> Group<Self>
    where
        Self: Sized;
}

macro_rules! group_ext {
    ($($ty:ty),+) => {$(
        impl Sealed for $ty {}
        impl GroupExt for $ty {
            fn constrain(self) -> Group<Self> {
                Group { regs: self }
            }
        }
    )+};
}

group_ext!(VADC_G0, VADC_G1);
#[cfg(any(
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800"
))]
group_ext!(VADC_G2, VADC_G3);

/// One conversion group.
pub struct Group<G> {
    regs: G,
}

/// Conversion channel. The const parameter is the channel index inside the group.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AdcChannel<const N: u8>;

impl<const N: u8> Channel<Vadc> for AdcChannel<N> {
    type ID = u8;

    fn channel() -> Self::ID {
        N
    }
}

impl<G> Group<G>
where
    G: Deref<Target = pac::vadc_g0::RegisterBlock>,
{
    /// Normal operation of the analog converter (`ARBCFG.ANONC` = 3).
    pub fn power_up(&self) {
        self.regs
            .arbcfg()
            .modify(|_, w| unsafe { w.anonc().bits(3) });
    }

    pub fn power_down(&self) {
        self.regs
            .arbcfg()
            .modify(|_, w| unsafe { w.anonc().bits(0) });
    }

    /// Master/slave synchronization. 0 is master, 1..=3 follow another group.
    pub fn set_sync(&self, mode: u8) -> Result<(), u8> {
        if mode > 3 {
            return Err(mode);
        }
        self.regs
            .synctr()
            .modify(|_, w| unsafe { w.stsel().bits(mode) });
        Ok(())
    }

    pub fn select_scan(&self, mask: u8) {
        self.regs.assel().modify(|_, w| {
            let bits = [
                mask & 1 != 0,
                mask & 2 != 0,
                mask & 4 != 0,
                mask & 8 != 0,
                mask & 16 != 0,
                mask & 32 != 0,
                mask & 64 != 0,
                mask & 128 != 0,
            ];
            if bits[0] {
                w.chsel0().set_bit();
            } else {
                w.chsel0().clear_bit();
            }
            if bits[1] {
                w.chsel1().set_bit();
            } else {
                w.chsel1().clear_bit();
            }
            if bits[2] {
                w.chsel2().set_bit();
            } else {
                w.chsel2().clear_bit();
            }
            if bits[3] {
                w.chsel3().set_bit();
            } else {
                w.chsel3().clear_bit();
            }
            if bits[4] {
                w.chsel4().set_bit();
            } else {
                w.chsel4().clear_bit();
            }
            if bits[5] {
                w.chsel5().set_bit();
            } else {
                w.chsel5().clear_bit();
            }
            if bits[6] {
                w.chsel6().set_bit();
            } else {
                w.chsel6().clear_bit();
            }
            if bits[7] {
                w.chsel7().set_bit();
            } else {
                w.chsel7().clear_bit();
            }
            w
        });
    }

    pub fn enable_queue(&self) {
        self.regs.qmr0().modify(|_, w| unsafe { w.engt().bits(1) });
    }

    /// Queue a channel and issue a software trigger.
    pub fn start(&self, channel: u8) -> Result<(), u8> {
        if channel > 7 {
            return Err(channel);
        }
        self.regs
            .qinr0()
            .write(|w| unsafe { w.reqchnr().bits(channel) });
        self.regs.qmr0().modify(|_, w| w.trev().set_bit());
        Ok(())
    }

    /// Result register for a channel, if the valid flag is set.
    pub fn result(&self, channel: usize) -> Option<u16> {
        if channel > 15 {
            return None;
        }
        let res = self.regs.res(channel).read();
        if res.vf().bit_is_set() {
            Some(res.result().bits())
        } else {
            None
        }
    }
}

impl<G, const N: u8> OneShot<Vadc, u16, AdcChannel<N>> for Group<G>
where
    G: Deref<Target = pac::vadc_g0::RegisterBlock>,
{
    type Error = u8;

    fn read(&mut self, _channel: &mut AdcChannel<N>) -> nb::Result<u16, Self::Error> {
        self.power_up();
        self.enable_queue();
        self.start(N).map_err(nb::Error::Other)?;
        loop {
            if let Some(value) = self.result(usize::from(N)) {
                return Ok(value);
            }
        }
    }
}
