use crate::device::{SCU_CLK, SCU_GENERAL, SCU_POWER, SCU_RESET};
/// PDIV for main PLL
const PLL_PDIV_XTAL_8MHZ: u32 = 1;

/// NDIV for main PLL
const PLL_NDIV_XTAL_8MHZ: u32 = 89;

/// K2DIV for main PLL
const PLL_K2DIV_XTAL_8MHZ: u32 = 2;

pub struct Scu {}

pub enum Status {
    /// Scu operations completed
    Ok,
    /// Scu operations failed or cannot be fulfilled.
    Error,
    /// Cannot complete request because another one is in progress
    Busy,
}

/// Triggers for global start of control of specified Ccu channels
pub enum CcuTrigger {
    Ccu40,
    Ccu41,
    Ccu42,
    Ccu43,
    Ccu80,
    Ccu81,
}

pub enum Trap {
    OscWdg,
    VcoLock,
    UsbVcoLock,
    ParityError,
    Brownout,
    UlpWdg,
    PerBridge0,
    PerBridge1,
    DieTempHigh,
    DieTempLow,
    EcatReset,
}

pub enum Parity {
    PsramMem,
    Dsram1Mem,
    Dsram2Mem,
    Usic0Mem,
    Usic1Mem,
    Usic2Mem,
    McanMem,
    PmuMem,
    UsbMem,
    EthTxMem,
    EthRxMem,
    SdmmcMem0,
    SdmmcMem1,
}

pub enum ResetReason {
    Porst,
    Swd,
    Pv,
    Sw,
    Lockup,
    Watchdog,
    ParityError,
}

pub enum NmiReq {
    WdtWarn,
    RtcPi,
    RtcAi,
    Eru00,
    Eru01,
    Eru02,
    Eru03,
}

pub enum PeripheralReset {
    Vadc,
    Dsd,
    Ccu40,
    Ccu41,
    Ccu42,
    Ccu80,
    Ccu81,
    Posif0,
    Posif1,
    Usic0,
    Eru1,
    Hrpwm0,
    Ccu43,
    Ledts0,
    Mcan,
    Dac,
    Sdmmc,
    Usic1,
    Usic2,
    Ports,
    Wdt,
    Eth0,
    Gpdma0,
    Gpdma1,
    Fce,
    Usb0,
    Ecat0,
    Ebu,
}

pub enum Clock {
    Usb,
    Mmc,
    Eth,
    Ebu,
    Ccu,
    Wdt,
}

impl From<Clock> for u32 {
    fn from(bits: Clock) -> Self {
        match bits {
            Clock::Usb => 0x01,
            Clock::Mmc => 0x02,
            Clock::Eth => 0x04,
            Clock::Ebu => 0x08,
            Clock::Ccu => 0x10,
            Clock::Wdt => 0x20,
        }
    }
}

#[cfg(not(feature = "xmc4500"))]
pub enum PeripheralClock {
    Vadc,
    Dsd,
    Ccu40,
    Ccu41,
    Ccu42,
    Ccu80,
    Ccu81,
    Posif0,
    Posif1,
    Usic0,
    Eru1,
    Hrpwm0,
    Ccu43,
    Ledts0,
    Mcan,
    Dac,
    Sdmmc,
    Usic1,
    Usic2,
    Ports,
    Wdt,
    Eth0,
    Gpdma0,
    Gpdma1,
    Fce,
    Usb0,
    Ecat0,
    Ebu,
}

pub enum SysClockSource {
    Ofi,
    Pll,
}

pub enum SysPllClockSource {
    Oschp,
    Ofi,
}

pub enum UsbClockSource {
    UsbPll,
    SysPll,
}

#[cfg(feature = "ecat")]
pub enum EcatClockSource {
    UsbPll,
    SysPll,
}

pub enum WdtClockSource {
    Ofi,
    Stdby,
    Pll,
}

pub enum ExtOutClockSource {
    Sys,
    Usb,
    Pll,
    #[cfg(any(feature = "xmc4100", feature = "xmc4200"))]
    Stdby,
}

pub enum RtcClockSource {
    Osi,
    Ulp,
}

pub enum StdbyClockSource {
    Osi,
    Osculp,
}

pub enum FofiCalibrationMode {
    Factory,
    Automatic,
}

pub enum BootMode {
    Normal,
    AscBsl,
    Bmi,
    CanBsl,
    PsramBoot,
    Abm0,
    Abm1,
    Fabm,
}

impl From<BootMode> for u8 {
    fn from(bits: BootMode) -> u8 {
        match bits {
            BootMode::Normal => 0,
            BootMode::AscBsl => 1,
            BootMode::Bmi => 2,
            BootMode::CanBsl => 3,
            BootMode::PsramBoot => 4,
            BootMode::Abm0 => 5,
            BootMode::Abm1 => 6,
            BootMode::Fabm => 7,
        }
    }
}

impl From<u8> for BootMode {
    fn from(bits: u8) -> BootMode {
        match bits {
            0 => BootMode::Normal,
            1 => BootMode::AscBsl,
            2 => BootMode::Bmi,
            3 => BootMode::CanBsl,
            4 => BootMode::PsramBoot,
            5 => BootMode::Abm0,
            6 => BootMode::Abm1,
            7 => BootMode::Fabm,
            _ => unimplemented!(),
        }
    }
}

pub enum SysPllMode {
    Disabled,
    Normal,
    Prescaler,
}

pub enum SleepModeConfig {
    SysClkFofi,
    SysClkPll,
    DisableUsb,
    EnableUsb,
    DisableSdmmc,
    EnableSdmmc,
    DisableEth,
    EnableEth,
    DisableEbu,
    EnableEbu,
    DisableCcu,
    EnableCcu,
    DisableWdt,
    EnableWdt,
}

pub enum DeepSleepModeConfig {
    SysClkFofi,
    SysClkPll,
    FlashPowerDown,
    PllPowerDown,
    VcoPowerDown,
    DisableUsb,
    EnableUsb,
    DisableSdmmc,
    EnableSdmmc,
    DisableEth,
    EnableEth,
    DisableEbu,
    EnableEbu,
    DisableCcu,
    EnableCcu,
    DisableWdt,
    EnableWdt,
}

pub enum PowerEvrStatus {
    Ok,
    Evr13OverVoltage,
}

pub enum CtrlStatus {
    NoActive,
    Active,
}

pub enum HibernateEvent {
    OnPosEdge,
    OnNegEdge,
    OnRtc,
    UlpWdg,
}

pub enum HibernateIo {
    Io0,
    Io1,
}

pub enum HibernatePinMode {
    InputPullNone,
    InputPullDown,
    InputPullUp,
    OutputPushPullHibCtrl,
    OutputPushPullWdtSrv,
    OutputPushPullGpio,
    OutputOpenDrainHibCtrl,
    OutputOpenDrainWdtSrv,
    OutputOpenDrainGpio,
}

pub enum HibernateIoOutputLevel {
    Low,
    High,
}

impl Scu {

    pub fn new() -> Self {
        Scu {}
    }
    // TODO [#68]: Add implementation to enabling out of range comparator.
    pub fn enable_out_of_range_comparator(&self, _group: u32, _channel: u32) {
        // let reg = get_reg!(SCU_GENERAL, gorcen);
        unimplemented!();
    }

    // TODO [#69]: Add implementation to disabling out of range comparator.
    pub fn disable_out_of_range_comparator(&self, _group: u32, _channel: u32) {
        unimplemented!();
    }

    pub fn enable_hibernate_domain(&self) {
        let scu = periph!(SCU_POWER);
        if scu.pwrstat.read().hiben().bit_is_clear() {
            scu.pwrset.write(|w| w.hib().set_bit());
            while scu.pwrstat.read().hiben().bit_is_clear() {}
        }
    }
    pub fn is_hibernate_domain_enabled(&self) -> bool {
        get_field!(SCU_POWER, pwrstat, hiben).bit_is_set()
            && !get_field!(SCU_RESET, rststat, hibrs).bit_is_set()
    }

    /// Access the currently programmed boot mode
    pub fn get_boot_mode(&self) -> BootMode {
        BootMode::from(get_field!(SCU_GENERAL, stcon, swcon).bits())
    }

    /// Program a new device boot mode. The newly set boot mode can be launched by a software reset after updating.
    pub fn set_boot_mode(&self, mode: BootMode) {
        set_field!(SCU_GENERAL, stcon, swcon, u8::from(mode));
    }

    /// Enable a specific clock in the Scu peripheral
    pub fn enable_clock(&self, clock: Clock) {
        set_reg!(SCU_CLK, clkset, u32::from(clock));
    }

    /// Disable a specific clock in the Scu peripheral
    pub fn disable_clock(&self, clock: Clock) {
        set_reg!(SCU_CLK, clkclr, u32::from(clock));
    }

    /// Check if a peripheral clock is enabled.
    pub fn is_clock_enabled(&self, clock: Clock) -> bool {
        return (get_reg!(SCU_CLK, clkstat) & u32::from(clock)) > 0;
    }

    #[cfg(not(feature = "xmc4500"))]
    pub fn gate_peripheral_clock(&self, _clock: PeripheralClock) {}

    #[cfg(not(feature = "xmc4500"))]
    pub fn ungate_peripheral_clock(&self, _clock: PeripheralClock) {}

    // TODO [#71]: Update assert_peripheral_reset to const fn once stable
    pub fn assert_peripheral_reset(&self, peripheral: PeripheralReset) {
        match peripheral {
            PeripheralReset::Wdt => {
                set!(SCU_RESET, prset2, wdtrs);
            }
            _ => unimplemented!(),
        };
    }

    // TODO [#72]: Update deassert_peripheral_reset to const fn once stable
    pub fn deassert_peripheral_reset(&self, peripheral: PeripheralReset) {
        match peripheral {
            PeripheralReset::Wdt => {
                set!(SCU_RESET, prclr2, wdtrs);
            }
            _ => unimplemented!(),
        };
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
