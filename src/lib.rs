//! Hardware abstraction for Infineon XMC4000 microcontrollers.
//!
//! The crate sits on the `xmc4xxx` peripheral access crates. Pick exactly one
//! device feature (`xmc4100` … `xmc4800`). Drivers follow the stm32-rs model:
//! extension traits consume a PAC peripheral, clocks come from [`scu`], and
//! `embedded-hal` 1.0 traits are implemented where a trait exists. Watchdog,
//! PWM, and ADC use `embedded-hal` 0.2, re-exported as [`hal_02`].

#![no_std]
#![allow(clippy::new_without_default)]
#![allow(dead_code)]

// Error if not target feature provided
#[cfg(not(any(
    feature = "xmc4100",
    feature = "xmc4200",
    feature = "xmc4300",
    feature = "xmc4400",
    feature = "xmc4500",
    feature = "xmc4700",
    feature = "xmc4800",
)))]
compile_error!("Target not provided. Give `--features <target>` is required.");

// Error if more than one target feature provided
#[cfg(any(
    all(feature = "xmc4100", feature = "xmc4200"),
    all(feature = "xmc4100", feature = "xmc4300"),
    all(feature = "xmc4100", feature = "xmc4400"),
    all(feature = "xmc4100", feature = "xmc4500"),
    all(feature = "xmc4100", feature = "xmc4700"),
    all(feature = "xmc4100", feature = "xmc4800"),
    all(feature = "xmc4200", feature = "xmc4300"),
    all(feature = "xmc4200", feature = "xmc4400"),
    all(feature = "xmc4200", feature = "xmc4500"),
    all(feature = "xmc4200", feature = "xmc4700"),
    all(feature = "xmc4200", feature = "xmc4800"),
    all(feature = "xmc4300", feature = "xmc4400"),
    all(feature = "xmc4300", feature = "xmc4500"),
    all(feature = "xmc4300", feature = "xmc4700"),
    all(feature = "xmc4300", feature = "xmc4800"),
    all(feature = "xmc4400", feature = "xmc4500"),
    all(feature = "xmc4400", feature = "xmc4700"),
    all(feature = "xmc4400", feature = "xmc4800"),
    all(feature = "xmc4500", feature = "xmc4700"),
    all(feature = "xmc4500", feature = "xmc4800"),
    all(feature = "xmc4700", feature = "xmc4800"),
))]
compile_error!("Multiple targets specified. Can only specify `--features <target>` once.");

#[macro_use]
pub mod macros;

pub mod delay;
pub mod prelude;
pub mod time;

pub use embedded_hal as hal;
pub use embedded_hal_02 as hal_02;
pub use embedded_hal_nb as hal_nb;
pub use nb;
pub use nb::block;

mod sealed {
    pub trait Sealed {}
}
#[allow(unused_imports)]
pub(crate) use sealed::Sealed;

#[cfg(feature = "xmc4100")]
pub use xmc4100;
#[cfg(feature = "xmc4100")]
pub use xmc4100 as pac;

#[cfg(feature = "xmc4200")]
pub use xmc4200;
#[cfg(feature = "xmc4200")]
pub use xmc4200 as pac;

#[cfg(feature = "xmc4300")]
pub use xmc4300;
#[cfg(feature = "xmc4300")]
pub use xmc4300 as pac;

#[cfg(feature = "xmc4400")]
pub use xmc4400;
#[cfg(feature = "xmc4400")]
pub use xmc4400 as pac;

#[cfg(feature = "xmc4500")]
pub use xmc4500;
#[cfg(feature = "xmc4500")]
pub use xmc4500 as pac;

#[cfg(feature = "xmc4700")]
pub use xmc4700;
#[cfg(feature = "xmc4700")]
pub use xmc4700 as pac;

#[cfg(feature = "xmc4800")]
pub use xmc4800;
#[cfg(feature = "xmc4800")]
pub use xmc4800 as pac;

#[cfg(feature = "device-selected")]
pub mod can;
#[cfg(feature = "device-selected")]
pub mod ccu40;
#[cfg(feature = "device-selected")]
pub mod ccu80;
#[cfg(feature = "device-selected")]
pub mod dac;
#[cfg(feature = "device-selected")]
pub mod dlr;

#[cfg(feature = "ecat")]
pub mod ecat;

#[cfg(all(
    feature = "device-selected",
    not(any(feature = "xmc4100", feature = "xmc4200"))
))]
pub mod eth;

#[cfg(feature = "device-selected")]
pub mod eru;
#[cfg(feature = "device-selected")]
pub mod fce;
#[cfg(feature = "device-selected")]
pub mod flash;
#[cfg(feature = "device-selected")]
pub mod gpdma;
#[cfg(any(feature = "xmc4100", feature = "xmc4200", feature = "xmc4400"))]
pub mod hrpwm;
#[cfg(feature = "device-selected")]
pub mod ledts;
#[cfg(feature = "device-selected")]
pub mod pmu;
#[cfg(feature = "device-selected")]
pub mod port;
#[cfg(all(feature = "device-selected", not(feature = "xmc4300")))]
pub mod posif;
#[cfg(feature = "device-selected")]
pub mod pref;
#[cfg(feature = "device-selected")]
pub mod rtc;

#[cfg(feature = "sdmmc")]
pub mod sdmmc;

#[cfg(feature = "device-selected")]
pub mod i2c;
#[cfg(feature = "device-selected")]
pub mod i2s;
#[cfg(feature = "device-selected")]
pub mod scu;
#[cfg(feature = "device-selected")]
pub mod serial;
#[cfg(feature = "device-selected")]
pub mod spi;
#[cfg(feature = "device-selected")]
pub mod usb;
#[cfg(feature = "device-selected")]
pub mod usic;
#[cfg(feature = "device-selected")]
pub mod vadc;
#[cfg(feature = "device-selected")]
pub mod wdt;
