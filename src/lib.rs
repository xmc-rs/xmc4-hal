#![no_std]
#![allow(non_camel_case_types)]
#![allow(clippy::new_without_default)]

#[macro_use]
pub mod macros;

#[cfg(feature = "xmc4100")]
pub use xmc4100;
#[cfg(feature = "xmc4100")]
pub use xmc4100 as device;

#[cfg(feature = "xmc4200")]
pub use xmc4200;
#[cfg(feature = "xmc4200")]
pub use xmc4200 as device;

#[cfg(feature = "xmc4300")]
pub use xmc4300;
#[cfg(feature = "xmc4300")]
pub use xmc4300 as device;

#[cfg(feature = "xmc4400")]
pub use xmc4400;
#[cfg(feature = "xmc4400")]
pub use xmc4400 as device;

#[cfg(feature = "xmc4500")]
pub use xmc4500;
#[cfg(feature = "xmc4500")]
pub use xmc4500 as device;

#[cfg(feature = "xmc4700")]
pub use xmc4700;
#[cfg(feature = "xmc4700")]
pub use xmc4700 as device;

#[cfg(feature = "xmc4800")]
pub use xmc4800;
#[cfg(feature = "xmc4800")]
pub use xmc4800 as device;

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

#[cfg(any(feature = "xmc4300", feature = "xmc4800"))]
pub mod ecat;

#[cfg(feature = "device-selected")]
pub mod eru;
#[cfg(feature = "device-selected")]
pub mod fce;
#[cfg(feature = "device-selected")]
pub mod flash;
#[cfg(feature = "device-selected")]
pub mod gpdma;
#[cfg(feature = "device-selected")]
pub mod hrpwm;
#[cfg(feature = "device-selected")]
pub mod ledts;
#[cfg(feature = "device-selected")]
pub mod pba;
#[cfg(feature = "device-selected")]
pub mod pmu;
#[cfg(feature = "device-selected")]
pub mod port;
#[cfg(feature = "device-selected")]
pub mod posif;
#[cfg(feature = "device-selected")]
pub mod ppb;
#[cfg(feature = "device-selected")]
pub mod pref;
#[cfg(feature = "device-selected")]
pub mod rtc;

#[cfg(any(feature = "xmc4300", feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
pub mod sdmmc;

#[cfg(feature = "device-selected")]
pub mod scu;
#[cfg(feature = "device-selected")]
pub mod usb;
#[cfg(feature = "device-selected")]
pub mod usic;
#[cfg(feature = "device-selected")]
pub mod vadc;
#[cfg(feature = "device-selected")]
pub mod wdt;

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
