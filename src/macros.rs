/// Get pointer to peripheral register set
macro_rules! periph {
    ($periph:ident) => {
        unsafe { &*$periph::ptr() }
    };
}

/// Set the bit of a register
macro_rules! set {
    ($periph:ident, $reg:ident, $bits:ident) => {
        let periph = periph!($periph);
        periph.$reg.write(|w| w.$bits().set_bit());
    };
}

/// Reset the bit of a register
macro_rules! clear {
    ($periph:ident, $reg:ident, $bits:ident) => {
        let periph = periph!($periph);
        periph.$reg.write(|w| w.$bits().clear_bit());
    };
}

macro_rules! set_reg {
    ($periph:ident, $reg:ident, $value:expr) => {
        let periph = periph!($periph);
        unsafe { periph.$reg.write(|w| w.bits($value)) };
    };
}

macro_rules! set_field {
    ($periph:ident, $reg:ident, $bits:ident, $value:expr) => {
        let periph = periph!($periph);
        unsafe { periph.$reg.write(|w| w.$bits().bits($value)) };
    };
}

macro_rules! get_reg {
    ($periph:ident, $reg:ident) => {
        unsafe { &*$periph::ptr() }.$reg.read().bits()
    };
}

macro_rules! get_field {
    ($periph:ident, $reg:ident, $field:ident) => {
        unsafe { &*$periph::ptr() }.$reg.read().$field()
    };
}
