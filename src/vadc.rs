use crate::device::VADC;

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
            _ => unimplemented!(),
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

impl From<u8> for StartMode {
    fn from(bits: u8) -> Self {
        match bits {
            0 => StartMode::WithoutInterruption,
            1 => StartMode::InterruptionResume,
            2 => StartMode::InterruptionStop,
            _ => unimplemented!(),
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

impl From<u8> for TriggerEdge {
    fn from(bits: u8) -> Self {
        match bits {
            0 => TriggerEdge::None,
            1 => TriggerEdge::Falling,
            2 => TriggerEdge::Rising,
            3 => TriggerEdge::Any,
            _ => unimplemented!(),
        }
    }
}

/// Different possibilities that can act as an external input for triggering conversion. The results of the chosen item
/// differ per channel.
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

impl From<u8> for TriggerInputSelect {
    fn from(bits: u8) -> Self {
        match bits {
            0 => TriggerInputSelect::A,
            1 => TriggerInputSelect::B,
            2 => TriggerInputSelect::C,
            3 => TriggerInputSelect::D,
            4 => TriggerInputSelect::E,
            5 => TriggerInputSelect::F,
            6 => TriggerInputSelect::G,
            7 => TriggerInputSelect::H,
            8 => TriggerInputSelect::I,
            9 => TriggerInputSelect::J,
            10 => TriggerInputSelect::K,
            11 => TriggerInputSelect::L,
            12 => TriggerInputSelect::M,
            13 => TriggerInputSelect::N,
            14 => TriggerInputSelect::O,
            15 => TriggerInputSelect::P,
            _ => unimplemented!(),
        }
    }
}

/// Different gating input possibilities that can gate conversion requests.
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

impl From<u8> for GateInputSelect {
    fn from(bits: u8) -> Self {
        match bits {
            0 => GateInputSelect::A,
            1 => GateInputSelect::B,
            2 => GateInputSelect::C,
            3 => GateInputSelect::D,
            4 => GateInputSelect::E,
            5 => GateInputSelect::F,
            6 => GateInputSelect::G,
            7 => GateInputSelect::H,
            8 => GateInputSelect::I,
            9 => GateInputSelect::J,
            10 => GateInputSelect::K,
            11 => GateInputSelect::L,
            12 => GateInputSelect::M,
            13 => GateInputSelect::N,
            14 => GateInputSelect::O,
            15 => GateInputSelect::P,
            _ => unimplemented!(),
        }
    }
}

/// Conditions for gating conversion requests.
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

impl From<u8> for GateMode {
    fn from(bits: u8) -> Self {
        match bits {
            0 => GateMode::Block,
            1 => GateMode::Ignore,
            2 => GateMode::ActiveHigh,
            3 => GateMode::ActiveLow,
            _ => unimplemented!(),
        }
    }
}

impl Vadc {
    /// Enable the clock for this peripheral.
    pub fn enable_module_clock(self) {
        set!(VADC, clc, disr);
    }

    /// Disable the clock for this peripheral.
    pub fn disable_module_clock(self) {
        clear!(VADC, clc, disr);
    }

    /// Allow peripheral to go to sleep if a request is received.
    pub fn enable_sleep_mode(self) {
        clear!(VADC, clc, edis);
    }

    /// Prevent peripheral to go to sleep if a request is received.
    pub fn disable_sleep_mode(self) {
        set!(VADC, clc, edis);
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
