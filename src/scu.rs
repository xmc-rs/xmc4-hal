use crate::pac::{SCU_CLK, SCU_GENERAL, SCU_INTERRUPT, SCU_POWER, SCU_RESET};
/// PDIV for main PLL
const PLL_PDIV_XTAL_8MHZ: u32 = 1;

/// NDIV for main PLL
const PLL_NDIV_XTAL_8MHZ: u32 = 89;

/// K2DIV for main PLL
const PLL_K2DIV_XTAL_8MHZ: u32 = 2;

pub struct Scu {}

#[derive(Copy, Clone, Debug)]
pub enum Status {
    /// Scu operations completed
    Ok,
    /// Scu operations failed or cannot be fulfilled.
    Error,
    /// Cannot complete request because another one is in progress
    Busy,
}

/// Triggers for global start of control of specified Ccu channels
#[derive(Copy, Clone, Debug)]
pub enum CcuTrigger {
    Ccu40,
    Ccu41,
    Ccu42,
    Ccu43,
    Ccu80,
    Ccu81,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub enum ResetReason {
    Porst,
    Swd,
    Pv,
    Sw,
    Lockup,
    Watchdog,
    ParityError,
}

#[derive(Copy, Clone, Debug)]
pub enum NmiReq {
    WdtWarn,
    RtcPi,
    RtcAi,
    Eru00,
    Eru01,
    Eru02,
    Eru03,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub enum SysClockSource {
    Ofi,
    Pll,
}

#[derive(Copy, Clone, Debug)]
pub enum SysPllClockSource {
    Oschp,
    Ofi,
}

#[derive(Copy, Clone, Debug)]
pub enum UsbClockSource {
    UsbPll,
    SysPll,
}

#[cfg(feature = "ecat")]
#[derive(Copy, Clone, Debug)]
pub enum EcatClockSource {
    UsbPll,
    SysPll,
}

#[derive(Copy, Clone, Debug)]
pub enum WdtClockSource {
    Ofi,
    Stdby,
    Pll,
}

#[derive(Copy, Clone, Debug)]
pub enum ExtOutClockSource {
    Sys,
    Usb,
    Pll,
    #[cfg(any(feature = "xmc4100", feature = "xmc4200"))]
    Stdby,
}

#[derive(Copy, Clone, Debug)]
pub enum RtcClockSource {
    Osi,
    Ulp,
}

#[derive(Copy, Clone, Debug)]
pub enum StdbyClockSource {
    Osi,
    Osculp,
}

#[derive(Copy, Clone, Debug)]
pub enum FofiCalibrationMode {
    Factory,
    Automatic,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub enum SysPllMode {
    Disabled,
    Normal,
    Prescaler,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub enum PowerEvrStatus {
    Ok,
    Evr13OverVoltage,
}

#[derive(Copy, Clone, Debug)]
pub enum CtrlStatus {
    NoActive,
    Active,
}

#[derive(Copy, Clone, Debug)]
pub enum HibernateEvent {
    OnPosEdge,
    OnNegEdge,
    OnRtc,
    UlpWdg,
}

#[derive(Copy, Clone, Debug)]
pub enum HibernateIo {
    Io0,
    Io1,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub enum HibernateIoOutputLevel {
    Low,
    High,
}

#[derive(Copy, Clone, Debug)]
pub enum InterruptEvent {
    WdtPreWarn,
    RtcPeriodic,
    RtcAlarm,
    DlrRequestOverrun,
    Lpaclr = 6,
    Lpacth0,
    Lpacth1,
    Lpacst,
    Lpacclr,
    Lpacset,
    Hintst,
    Hintclr,
    Hintset,
    Hdcrclr = 17,
    Hdcrset,
    Hdcr,
    Oscsictrl = 21,
    Osculctrl = 23,
    RtcCtr,
    RtcAtim0,
    RtcAtim1,
    RtcTim0,
    RtcTim1,
    RetentionMemory,
}

impl From<InterruptEvent> for u32 {
    fn from(bits: InterruptEvent) -> u32 {
        match bits {
            InterruptEvent::WdtPreWarn => 0,
            InterruptEvent::RtcPeriodic => 1,
            InterruptEvent::RtcAlarm => 2,
            InterruptEvent::DlrRequestOverrun => 3,
            InterruptEvent::Lpaclr => 6,
            InterruptEvent::Lpacth0 => 7,
            InterruptEvent::Lpacth1 => 8,
            InterruptEvent::Lpacst => 9,
            InterruptEvent::Lpacclr => 10,
            InterruptEvent::Lpacset => 11,
            InterruptEvent::Hintst => 12,
            InterruptEvent::Hintclr => 13,
            InterruptEvent::Hintset => 14,
            InterruptEvent::Hdcrclr => 17,
            InterruptEvent::Hdcrset => 18,
            InterruptEvent::Hdcr => 19,
            InterruptEvent::Oscsictrl => 21,
            InterruptEvent::Osculctrl => 23,
            InterruptEvent::RtcCtr => 24,
            InterruptEvent::RtcAtim0 => 25,
            InterruptEvent::RtcAtim1 => 26,
            InterruptEvent::RtcTim0 => 27,
            InterruptEvent::RtcTim1 => 28,
            InterruptEvent::RetentionMemory => 29,
        }
    }
}

impl From<u32> for InterruptEvent {
    fn from(bits: u32) -> InterruptEvent {
        match bits {
            0 => InterruptEvent::WdtPreWarn,
            1 => InterruptEvent::RtcPeriodic,
            2 => InterruptEvent::RtcAlarm,
            3 => InterruptEvent::DlrRequestOverrun,
            6 => InterruptEvent::Lpaclr,
            7 => InterruptEvent::Lpacth0,
            8 => InterruptEvent::Lpacth1,
            9 => InterruptEvent::Lpacst,
            10 => InterruptEvent::Lpacclr,
            11 => InterruptEvent::Lpacset,
            12 => InterruptEvent::Hintst,
            13 => InterruptEvent::Hintclr,
            14 => InterruptEvent::Hintset,
            17 => InterruptEvent::Hdcrclr,
            18 => InterruptEvent::Hdcrset,
            19 => InterruptEvent::Hdcr,
            21 => InterruptEvent::Oscsictrl,
            23 => InterruptEvent::Osculctrl,
            24 => InterruptEvent::RtcCtr,
            25 => InterruptEvent::RtcAtim0,
            26 => InterruptEvent::RtcAtim1,
            27 => InterruptEvent::RtcTim0,
            28 => InterruptEvent::RtcTim1,
            29 => InterruptEvent::RetentionMemory,
            _ => unimplemented!(),
        }
    }
}

impl Scu {
    pub fn new() -> Self {
        Scu {}
    }

    pub fn enable_event(&self, event: InterruptEvent) {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };

        scu.srmsk()
            .modify(|r, w| unsafe { w.bits(r.bits() | event as u32) });
    }

    pub fn disable_event(&self, event: InterruptEvent) {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };

        scu.srmsk()
            .modify(|r, w| unsafe { w.bits(r.bits() & !(event as u32)) });
    }

    pub fn trigger_event(&self, event: InterruptEvent) {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };
        scu.srset().write(|w| match event {
            InterruptEvent::WdtPreWarn => w.prwarn().set_bit(),
            InterruptEvent::RtcPeriodic => w.pi().set_bit(),
            InterruptEvent::RtcAlarm => w.ai().set_bit(),
            InterruptEvent::DlrRequestOverrun => w.dlrovr().set_bit(),
            InterruptEvent::Lpaclr => w.lpaccr().set_bit(),
            InterruptEvent::Lpacth0 => w.lpacth0().set_bit(),
            InterruptEvent::Lpacth1 => w.lpacth1().set_bit(),
            InterruptEvent::Lpacst => w.lpacst().set_bit(),
            InterruptEvent::Lpacclr => w.lpacclr().set_bit(),
            InterruptEvent::Lpacset => w.lpacset().set_bit(),
            InterruptEvent::Hintst => w.hintst().set_bit(),
            InterruptEvent::Hintclr => w.hintclr().set_bit(),
            InterruptEvent::Hintset => w.hintset().set_bit(),
            InterruptEvent::Hdcrclr => w.hdcrclr().set_bit(),
            InterruptEvent::Hdcrset => w.hdcrset().set_bit(),
            InterruptEvent::Hdcr => w.hdcr().set_bit(),
            InterruptEvent::Oscsictrl => w.oscsictrl().set_bit(),
            InterruptEvent::Osculctrl => w.osculctrl().set_bit(),
            InterruptEvent::RtcCtr => w.rtc_ctr().set_bit(),
            InterruptEvent::RtcAtim0 => w.rtc_atim0().set_bit(),
            InterruptEvent::RtcAtim1 => w.rtc_atim1().set_bit(),
            InterruptEvent::RtcTim0 => w.rtc_tim0().set_bit(),
            InterruptEvent::RtcTim1 => w.rtc_tim1().set_bit(),
            InterruptEvent::RetentionMemory => w.rmx().set_bit(),
        });
    }

    pub fn get_event_status() -> InterruptEvent {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };
        scu.srraw().read().bits().into()
    }

    pub fn enable_out_of_range_comparator(&self, group: u32, channel: u32) {
        let scu = unsafe { &*SCU_GENERAL::ptr() };

        match group {
            0 => match channel {
                6 => scu.g0orcen().write(|w| w.enorc6().set_bit()),
                7 => scu.g0orcen().write(|w| w.enorc7().set_bit()),
                _ => (),
            },
            1 => match channel {
                6 => scu.g1orcen().write(|w| w.enorc6().set_bit()),
                7 => scu.g1orcen().write(|w| w.enorc7().set_bit()),
                _ => (),
            },
            _ => (),
        }
    }

    pub fn disable_out_of_range_comparator(&self, group: u32, channel: u32) {
        let scu = unsafe { &*SCU_GENERAL::ptr() };

        match group {
            0 => match channel {
                6 => scu.g0orcen().write(|w| w.enorc6().clear_bit()),
                7 => scu.g0orcen().write(|w| w.enorc7().clear_bit()),
                _ => (),
            },
            1 => match channel {
                6 => scu.g1orcen().write(|w| w.enorc6().clear_bit()),
                7 => scu.g1orcen().write(|w| w.enorc7().clear_bit()),
                _ => (),
            },
            _ => (),
        }
    }

    pub fn enable_hibernate_domain(&self) {
        let scu = unsafe { &*SCU_POWER::ptr() };
        if scu.pwrstat().read().hiben().bit_is_clear() {
            scu.pwrset().write(|w| w.hib().set_bit());
            while scu.pwrstat().read().hiben().bit_is_clear() {}
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
        (get_reg!(SCU_CLK, clkstat) & u32::from(clock)) > 0
    }

    #[cfg(not(feature = "xmc4500"))]
    pub fn gate_peripheral_clock(&self, _clock: PeripheralClock) {}

    #[cfg(not(feature = "xmc4500"))]
    pub fn ungate_peripheral_clock(&self, _clock: PeripheralClock) {}

    pub fn assert_peripheral_reset(&self, peripheral: PeripheralReset) {
        match peripheral {
            PeripheralReset::Wdt => {
                set!(SCU_RESET, prset2, wdtrs);
            }
            _ => unimplemented!(),
        };
    }

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
