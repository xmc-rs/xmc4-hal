

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
