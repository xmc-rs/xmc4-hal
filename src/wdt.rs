//! # Watchdog Module
//!
//! The watchdog module allows for use of the internal peripheral module that can be serviced periodically to ensure
//! that the program is staying alive and not trailing off or getting stuck somewhere in a loop.

#![allow(dead_code)]

use crate::pac::WDT;
#[cfg(not(feature = "xmc4500"))]
use crate::scu::PeripheralClock;
use crate::scu::{Clock, PeripheralReset, Scu};

///
const ALARM_CLEAR: u32 = 2;

///Key applied to watchdog when serviced to reset timers.
const SERVICE_KEY: u32 = 0xABAD_CAFE;

/// Main Watchdog module to configure and utilize.
pub struct Wdt {
    /// Watchdog registers based on the peripheral registers crate.
    wdt: WDT,
    /// System Control Unit module to configure clock registers
    scu: Scu,
}

#[repr(u32)]
#[derive(PartialEq)]
pub enum Mode {
    Timeout,
    Prewarning,
}

impl From<Mode> for u32 {
    fn from(bits: Mode) -> Self {
        bits as u32
    }
}

impl From<u32> for Mode {
    fn from(bits: u32) -> Self {
        match bits {
            0 => Mode::Timeout,
            1 => Mode::Prewarning,
            _ => unimplemented!(),
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
    pub fn new(wdt: WDT, scu: Scu) -> Self {
        // Haven't resolved yet fully how i want to deal with this.
        // Need to do more reading.
        let w = Wdt { wdt: wdt, scu };
        w.enable();
        w
    }

    /// Activate the watchdog peripheral, including the clock.
    ///
    /// Must be called after `disable()` has been called to use watchdog again.
    pub fn enable(&self) {
        self.scu.enable_clock(Clock::Wdt);
        #[cfg(not(feature = "xmc4500"))]
        self.scu.ungate_peripheral_clock(PeripheralClock::Wdt);
        self.scu.deassert_peripheral_reset(PeripheralReset::Wdt);
    }

    /// Completely shut off the watchdog peripheral, including resetting registers and stopping clocks.
    pub fn disable(&self) {
        self.scu.assert_peripheral_reset(PeripheralReset::Wdt);
        #[cfg(not(feature = "xmc4500"))]
        self.scu.gate_peripheral_clock(PeripheralClock::Wdt);
        self.scu.disable_clock(Clock::Wdt);
    }

    pub fn set_window_bounds(&self, lower: u32, upper: u32) {
        self.wdt.wlb().write(|w| unsafe { w.wlb().bits(lower) });
        self.wdt.wub().write(|w| unsafe { w.wub().bits(upper) });
    }

    /// Start Watchdog peripheral so that it can be serviced.
    pub fn start(&self) {
        self.wdt.ctr().write(|w| w.enb().set_bit());
    }

    /// Stop the watchdog so that it does not need to be serviced.
    ///
    /// This should be called when debugging code and wanting to step through
    /// as the timer will not stop for the debugger.
    pub fn stop(&self) {
        self.wdt.ctr().write(|w| w.enb().clear_bit());
    }

    /// Set operating mode of watchdog.
    ///
    /// # Arugments
    /// - `mode` -- Timeout or Prewarning.
    pub fn set_mode(&self, mode: Mode) {
        if Mode::Timeout == mode {
            self.wdt.ctr().write(|w| w.pre().clear_bit());
        } else {
            self.wdt.ctr().write(|w| w.pre().set_bit());
        }
    }

    /// Set the window of time for the watchdog servicing.
    ///
    /// # Arguments
    /// - `pulse_width` -- Width of ticks to service watchdog.
    pub fn set_service_pulse_width(&self, pulse_width: u8) {
        self.wdt
            .ctr()
            .write(|w| unsafe { w.spw().bits(pulse_width) });
    }

    /// Activate or deactivate the debug mode of the peripheral when the CPU is in the HALT mode.
    ///
    /// # Arugments
    /// - `mode` -- Run for run during debugging and halting, Stop to stop the watchdog during debugging.
    pub fn set_debug_mode(&self, mode: DebugMode) {
        if DebugMode::Run == mode {
            self.wdt.ctr().write(|w| w.dsp().set_bit());
        } else {
            self.wdt.ctr().write(|w| w.dsp().clear_bit());
        }
    }

    /// Access current watchdog counter value.
    ///
    /// # Returns
    /// Current value of watchdog timer counter.
    pub fn get_counter(&self) -> u32 {
        self.wdt.tim().read().bits()
    }

    /// Alert watchdog to be serviced. This will reset the timer.
    pub fn service(&self) {
        self.wdt
            .srv()
            .write(|w| unsafe { w.srv().bits(SERVICE_KEY) });
    }

    /// Clear the previously trigged pre-warning alarm flag.
    pub fn clear_alarm(&self) {
        self.wdt.wdtclr().write(|w| unsafe { w.bits(ALARM_CLEAR) });
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Exists just to track coverage
    }
}
