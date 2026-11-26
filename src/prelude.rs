//! Re-exports the extension traits used to promote PAC peripherals into HAL types.

pub use crate::scu::ScuExt as _;
pub use crate::time::U32Ext;

pub use crate::can::CanExt as _;
pub use crate::ccu40::Ccu40Ext as _;
pub use crate::ccu80::Ccu80Ext as _;
pub use crate::dac::DacExt as _;
pub use crate::dlr::DlrExt as _;
pub use crate::eru::EruExt as _;
pub use crate::fce::FceExt as _;
pub use crate::gpdma::GpdmaExt as _;
pub use crate::hrpwm::HrpwmExt as _;
pub use crate::ledts::LedtsExt as _;
pub use crate::pmu::PmuExt as _;
pub use crate::port::PortExt as _;
pub use crate::posif::PosifExt as _;
pub use crate::pref::PrefExt as _;
pub use crate::rtc::RtcExt as _;
pub use crate::usb::UsbExt as _;
pub use crate::usic::UsicExt as _;
pub use crate::vadc::VadcExt as _;

#[cfg(feature = "ecat")]
pub use crate::ecat::EcatExt as _;
#[cfg(feature = "sdmmc")]
pub use crate::sdmmc::SdmmcExt as _;
