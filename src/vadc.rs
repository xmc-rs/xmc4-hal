use crate::device::{VADC};

pub trait VadcExt {
    fn constrain(self) -> Vadc;
}

impl VadcExt for Vadc {
    fn constrain(self) -> Vadc {
        Vadc {}
    }
}

pub struct Vadc {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

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
    SharedSR3
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
            _ => unimplemented!()
        }
    }
}

impl From<u8> for ServiceRequest {
    fn from(bits: u8) -> Self {
        match bits {
            0 => ServiceRequest::GroupSR0,
            1 => ServiceRequest::GroupSR1,
            2 => ServiceRequest::GroupSR2,
            3 => ServiceRequest::GroupSR3,
            4 => ServiceRequest::SharedSR0,
            5 => ServiceRequest::SharedSR1,
            6 => ServiceRequest::SharedSR2,
            7 => ServiceRequest::SharedSR3,
            _ => unimplemented!()
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
            _ => unimplemented!()
        }
    }
}

impl From<u8> for StartMode {
    fn from(bits: u8) -> Self {
        match bits {
            0 => StartMode::WithoutInterruption,
            1 => StartMode::InterruptionResume,
            2 => StartMode::InterruptionStop,
            _ => unimplemented!()
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
    Any
}

/// Different possibilities that can act as an external input for triggering conversion. The results of the chosen item
/// differ per channel.
#[derive(Debug, Clone, Copy)]
pub enum  TriggerInputSelect {
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
    P
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
    P
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
    ActiveLow
}

impl Vadc {

    /// Enable the clock for this peripheral.
    pub fn enable_module_clock(self) {
        let vadc = unsafe { &*VADC::ptr() };
        vadc.clc.modify(|_, w| w.disr().clear_bit());
    }

    /// Disable the clock for this peripheral.
    pub fn disable_module_clock(self) {
        let vadc = unsafe {&*VADC::ptr() };
        vadc.clc.modify(|_, w| w.disr().set_bit());
    }

    /// Allow peripheral to go to sleep if a request is received.
    pub fn enable_sleep_mode(self) {
        let vadc = unsafe {&*VADC::ptr() };
        vadc.clc.modify(|_, w| w.edis().clear_bit());
    }

    /// Prevent peripheral to go to sleep if a request is received.
    pub fn disable_sleep_mode(self) {
        let vadc = unsafe {&*VADC::ptr() };
        vadc.clc.modify(|_, w| w.edis().set_bit());
    }

    pub fn clock_init(self) {
        // TODO: Implement global clock initialization
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
