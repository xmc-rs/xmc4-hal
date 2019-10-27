/// Set the bit of a register
macro_rules! set {
    ($periph:ident, $reg:ident, $bits:ident) => {
        let periph = unsafe { &*$periph::ptr() };
        periph.$reg.modify(|_, w| w.$bits().set_bit());
    };
}

/// Reset the bit of a register
macro_rules! clear {
    ($periph:ident, $reg:ident, $bits:ident) => {
        let periph = unsafe { &*$periph::ptr() };
        periph.$reg.modify(|_, w| w.$bits().clear_bit());
    };
}

macro_rules! set_reg {
    ($periph:ident, $reg:ident, $value:expr) => {
        let periph = unsafe { &*$periph::ptr() };
        unsafe { periph.$reg.modify(|_, w| w.bits($value)) };
    };
}

macro_rules! set_field {
    ($periph:ident, $reg:ident, $bits:ident, $value:expr) => {
        let periph = unsafe { &*$periph::ptr() };
        unsafe {periph.$reg.modify(|_, w| w.$bits().bits($value))};
    };
}
