use crate::device::{WDT};

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
pub enum Mode {
    Timeout,
    Prewarning
}

// Todo: Verify the values of the enum
pub enum DebugMode {
    Stop,
    Run
}

pub enum EventMode {
    Interrupt,
    NmiRequest
}

pub enum Status {
    Success,
    Failure
}

impl Wdt {
    pub fn enable() {

    }

    pub fn disable() {

    }

    pub fn set_window_bounds(lower: u32, upper: u32) {
        let wdt = unsafe {&*WDT::ptr()};
        unsafe { wdt.wlb.modify(|_, w| w.bits(lower))};
    }

    pub fn start() {

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
