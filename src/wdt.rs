#![allow(dead_code)]

use crate::device::WDT;

const ALARM_CLEAR: u32 = 2;

const SERVICE_KEY: u32 = 0xABAD_CAFE;

pub trait WdtExt {
    fn constrain(self) -> Wdt;
}

impl WdtExt for Wdt {
    fn constrain(self) -> Wdt {
        Wdt {}
    }
}

pub struct Wdt {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

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

// Todo: Verify the values of the enum
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
    pub fn enable() {}

    pub fn disable() {}

    pub fn set_window_bounds(lower: u32, upper: u32) {
        set_reg!(WDT, wlb, lower);
        set_reg!(WDT, wub, upper);
    }

    pub fn start() {
        set!(WDT, ctr, enb);
    }

    pub fn stop() {
        clear!(WDT, ctr, enb);
    }

    pub fn set_mode(mode: Mode) {
        if Mode::Timeout == mode {
            clear!(WDT, ctr, pre);
        } else {
            set!(WDT, ctr, pre);
        }
    }

    pub fn set_service_pulse_width(pulse_width: u8) {
        set_field!(WDT, ctr, spw, pulse_width);
    }

    pub fn set_debug_mode(mode: DebugMode) {
        if DebugMode::Run == mode {
            set!(WDT, ctr, dsp);
        } else {
            clear!(WDT, ctr, dsp);
        }
    }

    pub fn get_counter() -> u32 {
        get_reg!(WDT, tim)
    }

    pub fn service() {
        set_reg!(WDT, srv, SERVICE_KEY);
    }

    pub fn clear_alarm() {
        set_reg!(WDT, wdtclr, ALARM_CLEAR);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
