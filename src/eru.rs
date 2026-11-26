//! Event Request Unit: event trigger logic (ETL) and output gating (OGU).

use core::ops::Deref;

use crate::pac::{self, ERU0, ERU1};
#[cfg(not(feature = "xmc4500"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;

const CHANNELS: usize = 4;

pub trait EruExt: Sealed {
    type Instance;
    fn constrain(self, scu: &mut Scu) -> Self::Instance;
}

impl Sealed for ERU0 {}
impl Sealed for ERU1 {}

impl EruExt for ERU0 {
    type Instance = Eru<ERU0>;
    fn constrain(self, scu: &mut Scu) -> Self::Instance {
        Eru::new(self, scu, false)
    }
}

impl EruExt for ERU1 {
    type Instance = Eru<ERU1>;
    fn constrain(self, scu: &mut Scu) -> Self::Instance {
        Eru::new(self, scu, true)
    }
}

/// Which ETL input combination produces the event.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Source {
    A = 0,
    B = 1,
    AOrB = 2,
    AAndB = 3,
}

/// Edge that sets the ETL status flag.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Edge {
    Disabled,
    Rising,
    Falling,
    Both,
}

/// Owned ERU instance. `ERU1` is the clock-gated block; `ERU0` is not.
pub struct Eru<R> {
    regs: R,
    gated: bool,
}

impl<R> Eru<R>
where
    R: Deref<Target = pac::eru0::RegisterBlock>,
{
    fn new(regs: R, scu: &mut Scu, gated: bool) -> Self {
        let eru = Self { regs, gated };
        eru.enable(scu);
        eru
    }

    pub fn enable(&self, scu: &mut Scu) {
        if self.gated {
            #[cfg(not(feature = "xmc4500"))]
            scu.ungate_peripheral_clock(PeripheralClock::Eru1);
            scu.deassert_peripheral_reset(PeripheralReset::Eru1);
        }
    }

    pub fn disable(&self, scu: &mut Scu) {
        if self.gated {
            scu.assert_peripheral_reset(PeripheralReset::Eru1);
            #[cfg(not(feature = "xmc4500"))]
            scu.gate_peripheral_clock(PeripheralClock::Eru1);
        }
    }

    fn channel(channel: usize) -> Result<usize, usize> {
        if channel < CHANNELS {
            Ok(channel)
        } else {
            Err(channel)
        }
    }

    /// Select the A and B inputs (each 0..=3) for an ETL channel.
    pub fn set_input(&self, channel: usize, input_a: u8, input_b: u8) -> Result<(), usize> {
        let channel = Self::channel(channel)?;
        if input_a > 3 || input_b > 3 {
            return Err(channel);
        }
        self.regs.exisel().modify(|_, w| unsafe {
            match channel {
                0 => {
                    w.exs0a().bits(input_a);
                    w.exs0b().bits(input_b);
                }
                1 => {
                    w.exs1a().bits(input_a);
                    w.exs1b().bits(input_b);
                }
                2 => {
                    w.exs2a().bits(input_a);
                    w.exs2b().bits(input_b);
                }
                _ => {
                    w.exs3a().bits(input_a);
                    w.exs3b().bits(input_b);
                }
            }
            w
        });
        Ok(())
    }

    /// Combine the selected inputs, optionally inverting each one.
    pub fn set_source(
        &self,
        channel: usize,
        source: Source,
        invert_a: bool,
        invert_b: bool,
    ) -> Result<(), usize> {
        let channel = Self::channel(channel)?;
        self.regs.exicon(channel).modify(|_, w| unsafe {
            w.ss().bits(source as u8);
            if invert_a {
                w.na().set_bit();
            } else {
                w.na().clear_bit();
            }
            if invert_b {
                w.nb().set_bit();
            } else {
                w.nb().clear_bit();
            }
            w
        });
        Ok(())
    }

    pub fn set_edge(&self, channel: usize, edge: Edge) -> Result<(), usize> {
        let channel = Self::channel(channel)?;
        self.regs.exicon(channel).modify(|_, w| {
            match edge {
                Edge::Disabled => {
                    w.re().clear_bit();
                    w.fe().clear_bit();
                }
                Edge::Rising => {
                    w.re().set_bit();
                    w.fe().clear_bit();
                }
                Edge::Falling => {
                    w.re().clear_bit();
                    w.fe().set_bit();
                }
                Edge::Both => {
                    w.re().set_bit();
                    w.fe().set_bit();
                }
            }
            w
        });
        Ok(())
    }

    pub fn set_status_flag(&self, channel: usize) -> Result<(), usize> {
        let channel = Self::channel(channel)?;
        self.regs.exicon(channel).modify(|_, w| w.fl().set_bit());
        Ok(())
    }

    pub fn clear_status_flag(&self, channel: usize) -> Result<(), usize> {
        let channel = Self::channel(channel)?;
        self.regs.exicon(channel).modify(|_, w| w.fl().clear_bit());
        Ok(())
    }

    pub fn status_flag(&self, channel: usize) -> Result<bool, usize> {
        let channel = Self::channel(channel)?;
        Ok(self.regs.exicon(channel).read().fl().bit_is_set())
    }

    /// `true` lets the event logic rebuild the flag. `false` leaves it to software.
    pub fn set_status_flag_hardware(&self, channel: usize, hardware: bool) -> Result<(), usize> {
        let channel = Self::channel(channel)?;
        self.regs.exicon(channel).modify(|_, w| {
            if hardware {
                w.ld().set_bit()
            } else {
                w.ld().clear_bit()
            }
        });
        Ok(())
    }

    /// Route the ETL trigger pulse to an OGU channel.
    pub fn enable_output_trigger(&self, channel: usize, ogu: u8) -> Result<(), usize> {
        let channel = Self::channel(channel)?;
        if ogu > 3 {
            return Err(channel);
        }
        self.regs.exicon(channel).modify(|_, w| unsafe {
            w.ocs().bits(ogu);
            w.pe().set_bit()
        });
        Ok(())
    }

    pub fn disable_output_trigger(&self, channel: usize) -> Result<(), usize> {
        let channel = Self::channel(channel)?;
        self.regs.exicon(channel).modify(|_, w| w.pe().clear_bit());
        Ok(())
    }

    /// Pattern-detect on the ETL flags selected by `inputs` (bit n is ETL n).
    pub fn enable_pattern_detection(&self, channel: usize, inputs: u8) -> Result<(), usize> {
        let channel = Self::channel(channel)?;
        self.regs.exocon(channel).modify(|_, w| {
            if inputs & 1 != 0 {
                w.ipen0().set_bit();
            } else {
                w.ipen0().clear_bit();
            }
            if inputs & 2 != 0 {
                w.ipen1().set_bit();
            } else {
                w.ipen1().clear_bit();
            }
            if inputs & 4 != 0 {
                w.ipen2().set_bit();
            } else {
                w.ipen2().clear_bit();
            }
            if inputs & 8 != 0 {
                w.ipen3().set_bit();
            } else {
                w.ipen3().clear_bit();
            }
            w.geen().set_bit()
        });
        Ok(())
    }

    pub fn disable_pattern_detection(&self, channel: usize) -> Result<(), usize> {
        let channel = Self::channel(channel)?;
        self.regs
            .exocon(channel)
            .modify(|_, w| w.geen().clear_bit());
        Ok(())
    }

    /// Peripheral trigger 0 disables the function. 1..=3 select OGUy1..=OGUy3.
    pub fn set_peripheral_trigger(&self, channel: usize, trigger: u8) -> Result<(), usize> {
        let channel = Self::channel(channel)?;
        if trigger > 3 {
            return Err(channel);
        }
        self.regs
            .exocon(channel)
            .modify(|_, w| unsafe { w.iss().bits(trigger) });
        Ok(())
    }

    /// Service-request gating: 0 off, 1 on trigger, 2 trigger and match, 3 trigger and mismatch.
    pub fn set_service_request(&self, channel: usize, mode: u8) -> Result<(), usize> {
        let channel = Self::channel(channel)?;
        if mode > 3 {
            return Err(channel);
        }
        self.regs
            .exocon(channel)
            .modify(|_, w| unsafe { w.gp().bits(mode) });
        Ok(())
    }
}
