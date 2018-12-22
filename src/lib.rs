#[cfg(feature = "xmc4100")]
pub use xmc4100;
#[cfg(feature = "xmc4100")]
pub use xmc4100 as device;

pub mod scu;
pub mod time;
