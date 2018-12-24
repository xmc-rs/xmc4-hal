#[cfg(feature = "xmc4100")]
pub use xmc4100;
#[cfg(feature = "xmc4100")]
pub use xmc4100 as device;

#[cfg(feature = "xmc4200")]
pub use xmc4200;
#[cfg(feature = "xmc4200")]
pub use xmc4200 as device;

#[cfg(feature = "xmc4500")]
pub use xmc4500;
#[cfg(feature = "xmc4500")]
pub use xmc4500 as device;

#[cfg(feature = "xmc4700")]
pub use xmc4700;
#[cfg(feature = "xmc4700")]
pub use xmc4700 as device;

pub mod rtc;
pub mod scu;
