#![allow(dead_code)]

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
        set_reg!(WDT, wlb, lower);
        set_reg!(WDT, wub, upper);
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
