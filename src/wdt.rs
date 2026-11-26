//! Watchdog timer.
//!
//! The counter must be serviced inside its window or the device resets.

#![allow(dead_code)]

use crate::pac::WDT;
#[cfg(not(feature = "xmc4500"))]
use crate::scu::PeripheralClock;
use crate::scu::{Clock, PeripheralReset, Scu};
use crate::sealed::Sealed;
use embedded_hal_02::watchdog::{Watchdog, WatchdogEnable};

/// Bit value that clears the pre-warning alarm.
const ALARM_CLEAR: u32 = 2;

/// Magic value written to the service register.
const SERVICE_KEY: u32 = 0xABAD_CAFE;

/// Owned watchdog.
pub struct Wdt {
    wdt: WDT,
}

pub trait WdtExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Wdt;
}

impl Sealed for WDT {}

impl WdtExt for WDT {
    fn constrain(self, scu: &mut Scu) -> Wdt {
        Wdt::new(self, scu)
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Timeout = 0,
    Prewarning = 1,
}

impl TryFrom<u32> for Mode {
    type Error = u32;

    fn try_from(bits: u32) -> Result<Self, Self::Error> {
        match bits {
            0 => Ok(Mode::Timeout),
            1 => Ok(Mode::Prewarning),
            other => Err(other),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum DebugMode {
    Stop,
    Run,
}

#[derive(Copy, Clone, Debug)]
pub enum EventMode {
    Interrupt,
    NmiRequest,
}

#[derive(Copy, Clone, Debug)]
pub enum Status {
    Success,
    Failure,
}

impl Wdt {
    /// Enable the watchdog clock and take it out of reset.
    pub fn new(wdt: WDT, scu: &mut Scu) -> Self {
        let w = Wdt { wdt };
        w.enable(scu);
        w
    }

    /// Activate the watchdog peripheral, including the clock.
    pub fn enable(&self, scu: &mut Scu) {
        scu.enable_clock(Clock::Wdt);
        #[cfg(not(feature = "xmc4500"))]
        scu.ungate_peripheral_clock(PeripheralClock::Wdt);
        scu.deassert_peripheral_reset(PeripheralReset::Wdt);
    }

    /// Reset the watchdog and stop its clock.
    pub fn disable(&self, scu: &mut Scu) {
        scu.assert_peripheral_reset(PeripheralReset::Wdt);
        #[cfg(not(feature = "xmc4500"))]
        scu.gate_peripheral_clock(PeripheralClock::Wdt);
        scu.disable_clock(Clock::Wdt);
    }

    pub fn set_window_bounds(&self, lower: u32, upper: u32) {
        self.wdt.wlb().write(|w| unsafe { w.wlb().bits(lower) });
        self.wdt.wub().write(|w| unsafe { w.wub().bits(upper) });
    }

    /// Start the watchdog so that it must be serviced.
    pub fn start(&self) {
        self.wdt.ctr().modify(|_, w| w.enb().set_bit());
    }

    /// Stop the watchdog so that it does not need to be serviced.
    pub fn stop(&self) {
        self.wdt.ctr().modify(|_, w| w.enb().clear_bit());
    }

    /// Timeout mode resets on expiry. Pre-warning mode raises an alarm first.
    pub fn set_mode(&self, mode: Mode) {
        self.wdt.ctr().modify(|_, w| {
            if mode == Mode::Timeout {
                w.pre().clear_bit()
            } else {
                w.pre().set_bit()
            }
        });
    }

    /// Width, in WDT clock ticks, of the service pulse.
    pub fn set_service_pulse_width(&self, pulse_width: u8) {
        self.wdt
            .ctr()
            .modify(|_, w| unsafe { w.spw().bits(pulse_width) });
    }

    /// Keep the watchdog running, or stop it, while the CPU is halted.
    pub fn set_debug_mode(&self, mode: DebugMode) {
        self.wdt.ctr().modify(|_, w| {
            if mode == DebugMode::Run {
                w.dsp().set_bit()
            } else {
                w.dsp().clear_bit()
            }
        });
    }

    /// Current watchdog counter.
    pub fn get_counter(&self) -> u32 {
        self.wdt.tim().read().bits()
    }

    /// Service the watchdog and restart its counter.
    pub fn service(&self) {
        self.wdt
            .srv()
            .write(|w| unsafe { w.srv().bits(SERVICE_KEY) });
    }

    /// Clear the pre-warning alarm.
    pub fn clear_alarm(&self) {
        self.wdt.wdtclr().write(|w| unsafe { w.bits(ALARM_CLEAR) });
    }
}

impl Watchdog for Wdt {
    fn feed(&mut self) {
        self.service();
    }
}

impl WatchdogEnable for Wdt {
    type Time = u32;

    fn start<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        self.set_window_bounds(0, period.into());
        Wdt::start(self);
    }
}
