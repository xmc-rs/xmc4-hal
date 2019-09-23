#![no_std]
#![allow(non_camel_case_types)]

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
pub mod rtc;
#[cfg(feature = "device-selected")]
pub mod scu;
