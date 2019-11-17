#![allow(dead_code)]

use crate::device::wdt::RegisterBlock;
use crate::device::WDT;
use crate::scu::{Clock, PeripheralClock, PeripheralReset, Scu};

const ALARM_CLEAR: u32 = 2;

const SERVICE_KEY: u32 = 0xABAD_CAFE;

pub struct Wdt {
    wdt: *const RegisterBlock,
    scu: Scu,
}

// Todo: Verify the values of the enum
#[derive(PartialEq)]
pub enum Mode {
    Timeout,
    Prewarning,
}

impl From<Mode> for u32 {
    fn from(bits: Mode) -> Self {
        match bits {
            Mode::Timeout => 0,
            Mode::Prewarning => 1,
        }
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

#[derive(PartialEq)]
pub enum DebugMode {
    Stop,
    Run,
}

pub enum EventMode {
    Interrupt,
    NmiRequest,
}

pub enum Status {
    Success,
    Failure,
}

impl Wdt {
    pub fn new(scu: Scu) -> Wdt {
        // Haven't resolved yet fully how i want to deal with this.
        // Need to do more reading.
        let w = Wdt {
            wdt: WDT::ptr(),
            scu,
        };
        w.enable();
        w
    }

    // TODO [#66]: Implement Scu API's for watchdog enable
    pub fn enable(&self) {
        self.scu.enable_clock(Clock::Wdt);
        self.scu.ungate_peripheral_clock(PeripheralClock::Wdt);
        self.scu.deassert_peripheral_reset(PeripheralReset::Wdt);
    }

    // TODO Implement Scu API's for watchdog disable
    pub fn disable(&self) {
        self.scu.assert_peripheral_reset(PeripheralReset::Wdt);
        self.scu.gate_peripheral_clock(PeripheralClock::Wdt);
        self.scu.disable_clock(Clock::Wdt);
    }

    pub fn set_window_bounds(self, lower: u32, upper: u32) {
        set_reg!(WDT, wlb, lower);
        set_reg!(WDT, wub, upper);
    }

    pub fn start(self) {
        set!(WDT, ctr, enb);
    }

    pub fn stop(self) {
        clear!(WDT, ctr, enb);
    }

    pub fn set_mode(self, mode: Mode) {
        if Mode::Timeout == mode {
            clear!(WDT, ctr, pre);
        } else {
            set!(WDT, ctr, pre);
        }
    }

    pub fn set_service_pulse_width(self, pulse_width: u8) {
        set_field!(WDT, ctr, spw, pulse_width);
    }

    pub fn set_debug_mode(self, mode: DebugMode) {
        if DebugMode::Run == mode {
            set!(WDT, ctr, dsp);
        } else {
            clear!(WDT, ctr, dsp);
        }
    }

    pub fn get_counter(self) -> u32 {
        get_reg!(WDT, tim)
    }

    pub fn service(self) {
        set_reg!(WDT, srv, SERVICE_KEY);
    }

    pub fn clear_alarm(self) {
        set_reg!(WDT, wdtclr, ALARM_CLEAR);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Exists just to track coverage
    }
}
