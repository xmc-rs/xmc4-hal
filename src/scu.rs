use crate::pac::{SCU_CLK, SCU_GENERAL, SCU_INTERRUPT, SCU_POWER, SCU_RESET, SCU_TRAP};
/// PDIV for main PLL
const PLL_PDIV_XTAL_8MHZ: u32 = 1;

/// NDIV for main PLL
const PLL_NDIV_XTAL_8MHZ: u32 = 89;

/// K2DIV for main PLL
const PLL_K2DIV_XTAL_8MHZ: u32 = 2;

#[derive(Debug)]
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
    #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
    Dsd,
    Ccu40,
    Ccu41,
    #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
    Ccu42,
    Ccu80,
    #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
    Ccu81,
    Posif0,
    #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
    Posif1,
    Usic0,
    Eru1,
    Hrpwm0,
    #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
    Ccu43,
    Ledtscu0,
    Mcan,
    Dac,
    #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
    Mmci,
    Usic1,
    #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
    Usic2,
    Pports,
    Wdt,
    #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
    Eth0,
    Dma0,
    #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
    Dma1,
    Fce,
    Usb0,
    #[cfg(any(feature = "xmc4300", feature = "xmc4800"))]
    Ecat0,
    #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
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

#[repr(u8)]
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
        bits as u8
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
    #[cfg(not(feature = "xmc4300"))]
    #[cfg(not(feature = "xmc4500"))]
    #[cfg(not(feature = "xmc4700"))]
    Lpaclr = 6,
    #[cfg(not(feature = "xmc4300"))]
    #[cfg(not(feature = "xmc4500"))]
    #[cfg(not(feature = "xmc4700"))]
    Lpacth0,
    #[cfg(not(feature = "xmc4300"))]
    #[cfg(not(feature = "xmc4500"))]
    #[cfg(not(feature = "xmc4700"))]
    Lpacth1,
    #[cfg(not(feature = "xmc4300"))]
    #[cfg(not(feature = "xmc4500"))]
    #[cfg(not(feature = "xmc4700"))]
    Lpacst,
    #[cfg(not(feature = "xmc4300"))]
    #[cfg(not(feature = "xmc4500"))]
    #[cfg(not(feature = "xmc4700"))]
    Lpacclr,
    #[cfg(not(feature = "xmc4300"))]
    #[cfg(not(feature = "xmc4500"))]
    #[cfg(not(feature = "xmc4700"))]
    Lpacset,
    #[cfg(not(feature = "xmc4300"))]
    #[cfg(not(feature = "xmc4500"))]
    #[cfg(not(feature = "xmc4700"))]
    Hintst,
    #[cfg(not(feature = "xmc4300"))]
    #[cfg(not(feature = "xmc4500"))]
    #[cfg(not(feature = "xmc4700"))]
    Hintclr,
    #[cfg(not(feature = "xmc4300"))]
    #[cfg(not(feature = "xmc4500"))]
    #[cfg(not(feature = "xmc4700"))]
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
        bits as u32
    }
}

impl From<u32> for InterruptEvent {
    fn from(bits: u32) -> InterruptEvent {
        match bits {
            0 => InterruptEvent::WdtPreWarn,
            1 => InterruptEvent::RtcPeriodic,
            2 => InterruptEvent::RtcAlarm,
            3 => InterruptEvent::DlrRequestOverrun,
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4700"))]
            6 => InterruptEvent::Lpaclr,
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            7 => InterruptEvent::Lpacth0,
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            8 => InterruptEvent::Lpacth1,
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            9 => InterruptEvent::Lpacst,
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            10 => InterruptEvent::Lpacclr,
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            11 => InterruptEvent::Lpacset,
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            12 => InterruptEvent::Hintst,
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            13 => InterruptEvent::Hintclr,
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
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
        scu.srset().write(|w| unsafe { w.bits(event as u32) });
    }

    pub fn get_event_status(&self) -> InterruptEvent {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };
        scu.srraw().read().bits().into()
    }

    pub fn clear_event_status(&self, event: InterruptEvent) {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };
        scu.srclr().write(|w| unsafe { w.bits(event as u32) });
    }

    pub fn enable_out_of_range_comparator(&self, group: u32, channel: u32) {
        let scu = unsafe { &*SCU_GENERAL::ptr() };

        match group {
            0 => match channel {
                6 => scu.g0orcen().write(|w| w.enorc6().set_bit()),
                7 => scu.g0orcen().write(|w| w.enorc7().set_bit()),
                _ => unreachable!(),
            },
            1 => match channel {
                6 => scu.g1orcen().write(|w| w.enorc6().set_bit()),
                7 => scu.g1orcen().write(|w| w.enorc7().set_bit()),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
    }

    pub fn disable_out_of_range_comparator(&self, group: u32, channel: u32) {
        let scu = unsafe { &*SCU_GENERAL::ptr() };

        match group {
            0 => match channel {
                6 => scu.g0orcen().write(|w| w.enorc6().clear_bit()),
                7 => scu.g0orcen().write(|w| w.enorc7().clear_bit()),
                _ => unreachable!(),
            },
            1 => match channel {
                6 => scu.g1orcen().write(|w| w.enorc6().clear_bit()),
                7 => scu.g1orcen().write(|w| w.enorc7().clear_bit()),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
    }

    pub fn read_gpr(&self, index: u32) -> u32 {
        let scu = unsafe { &*SCU_GENERAL::ptr() };
        match index {
            0 => scu.gpr0().read().bits(),
            1 => scu.gpr1().read().bits(),
            _ => unimplemented!(),
        }
    }

    pub fn write_gpr(&self, index: u32, data: u32) {
        let scu = unsafe { &*SCU_GENERAL::ptr() };
        match index {
            0 => scu.gpr0().write(|w| unsafe { w.bits(data) }),
            1 => scu.gpr1().write(|w| unsafe { w.bits(data) }),
            _ => unimplemented!(),
        };
    }

    pub fn enable_hibernate_domain(&self) {
        let scu = unsafe { &*SCU_POWER::ptr() };
        if scu.pwrstat().read().hiben().bit_is_clear() {
            scu.pwrset().write(|w| w.hib().set_bit());
            while scu.pwrstat().read().hiben().bit_is_clear() {}
        }
    }
    pub fn is_hibernate_domain_enabled(&self) -> bool {
        let power = unsafe { &*SCU_POWER::ptr() };
        let reset = unsafe { &*SCU_RESET::ptr() };

        power.pwrstat().read().hiben().bit_is_set() && reset.rststat().read().hibrs().bit_is_set()
    }

    /// Access the currently programmed boot mode
    pub fn get_boot_mode(&self) -> BootMode {
        let general = unsafe { &*SCU_GENERAL::ptr() };

        BootMode::from(general.stcon().read().swcon().bits())
    }

    /// Program a new device boot mode. The newly set boot mode can be launched by a software reset after updating.
    pub fn set_boot_mode(&self, mode: BootMode) {
        let general = unsafe { &*SCU_GENERAL::ptr() };

        general
            .stcon()
            .write(|w| unsafe { w.swcon().bits(u8::from(mode)) });
    }

    /// Enable a specific clock in the Scu peripheral
    pub fn enable_clock(&self, clock: Clock) {
        let clk = unsafe { &*SCU_CLK::ptr() };
        clk.clkset().write(|w| unsafe { w.bits(u32::from(clock)) });
    }

    /// Disable a specific clock in the Scu peripheral
    pub fn disable_clock(&self, clock: Clock) {
        let clk = unsafe { &*SCU_CLK::ptr() };
        clk.clkclr().write(|w| unsafe { w.bits(u32::from(clock)) });
    }

    /// Check if a peripheral clock is enabled.
    pub fn is_clock_enabled(&self, clock: Clock) -> bool {
        let clk = unsafe { &*SCU_CLK::ptr() };
        (clk.clkstat().read().bits() & u32::from(clock)) > 0
    }

    #[cfg(not(feature = "xmc4500"))]
    pub fn gate_peripheral_clock(&self, clock: PeripheralClock) {
        let scu = unsafe { &*SCU_CLK::ptr() };

        match clock {
            PeripheralClock::Vadc => scu.cgatset0().write(|w| w.vadc().set_bit()),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Dsd => scu.cgatset0().write(|w| w.dsd().set_bit()),
            PeripheralClock::Ccu40 => scu.cgatset0().write(|w| w.ccu40().set_bit()),
            PeripheralClock::Ccu41 => scu.cgatset0().write(|w| w.ccu41().set_bit()),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ccu42 => scu.cgatset0().write(|w| w.ccu42().set_bit()),
            PeripheralClock::Ccu80 => scu.cgatset0().write(|w| w.ccu80().set_bit()),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ccu81 => scu.cgatset0().write(|w| w.ccu81().set_bit()),
            PeripheralClock::Posif0 => scu.cgatset0().write(|w| w.posif0().set_bit()),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Posif1 => scu.cgatset0().write(|w| w.posif1().set_bit()),
            PeripheralClock::Usic0 => scu.cgatset0().write(|w| w.usic0().set_bit()),
            PeripheralClock::Eru1 => scu.cgatset0().write(|w| w.eru1().set_bit()),
            PeripheralClock::Hrpwm0 => unimplemented!(), // Appears to be missing from SVD
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ccu43 => scu.cgatset1().write(|w| w.ccu43().set_bit()),
            PeripheralClock::Ledtscu0 => scu.cgatset1().write(|w| w.ledtscu0().set_bit()),
            PeripheralClock::Mcan => scu.cgatset1().write(|w| w.mcan0().set_bit()),
            PeripheralClock::Dac => scu.cgatset1().write(|w| w.dac().set_bit()),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Mmci => scu.cgatset1().write(|w| w.mmci().set_bit()),
            PeripheralClock::Usic1 => scu.cgatset1().write(|w| w.usic1().set_bit()),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Usic2 => scu.cgatset1().write(|w| w.usic2().set_bit()),
            PeripheralClock::Pports => scu.cgatset1().write(|w| w.pports().set_bit()),
            PeripheralClock::Wdt => scu.cgatset2().write(|w| w.wdt().set_bit()),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Eth0 => scu.cgatset2().write(|w| w.eth0().set_bit()),
            PeripheralClock::Dma0 => scu.cgatset2().write(|w| w.dma0().set_bit()),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Dma1 => scu.cgatset2().write(|w| w.dma1().set_bit()),
            PeripheralClock::Fce => scu.cgatset2().write(|w| w.fce().set_bit()),
            PeripheralClock::Usb0 => scu.cgatset2().write(|w| w.usb().set_bit()),
            #[cfg(any(feature = "xmc4300", feature = "xmc4800"))]
            PeripheralClock::Ecat0 => scu.cgatset2().write(|w| w.ecat0().set_bit()),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ebu => scu.cgatset3().write(|w| w.ebu().set_bit()),
        };
    }

    #[cfg(not(feature = "xmc4500"))]
    pub fn ungate_peripheral_clock(&self, clock: PeripheralClock) {
        let scu = unsafe { &*SCU_CLK::ptr() };

        match clock {
            PeripheralClock::Vadc => scu.cgatclr0().write(|w| w.vadc().set_bit()),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Dsd => scu.cgatclr0().write(|w| w.dsd().set_bit()),
            PeripheralClock::Ccu40 => scu.cgatclr0().write(|w| w.ccu40().set_bit()),
            PeripheralClock::Ccu41 => scu.cgatclr0().write(|w| w.ccu41().set_bit()),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ccu42 => scu.cgatclr0().write(|w| w.ccu42().set_bit()),
            PeripheralClock::Ccu80 => scu.cgatclr0().write(|w| w.ccu80().set_bit()),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ccu81 => scu.cgatclr0().write(|w| w.ccu81().set_bit()),
            PeripheralClock::Posif0 => scu.cgatclr0().write(|w| w.posif0().set_bit()),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Posif1 => scu.cgatclr0().write(|w| w.posif1().set_bit()),
            PeripheralClock::Usic0 => scu.cgatclr0().write(|w| w.usic0().set_bit()),
            PeripheralClock::Eru1 => scu.cgatclr0().write(|w| w.eru1().set_bit()),
            PeripheralClock::Hrpwm0 => unimplemented!(), // Appears to be missing from SVD
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ccu43 => scu.cgatclr1().write(|w| w.ccu43().set_bit()),
            PeripheralClock::Ledtscu0 => scu.cgatclr1().write(|w| w.ledtscu0().set_bit()),
            PeripheralClock::Mcan => scu.cgatclr1().write(|w| w.mcan0().set_bit()),
            PeripheralClock::Dac => scu.cgatclr1().write(|w| w.dac().set_bit()),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Mmci => scu.cgatclr1().write(|w| w.mmci().set_bit()),
            PeripheralClock::Usic1 => scu.cgatclr1().write(|w| w.usic1().set_bit()),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Usic2 => scu.cgatclr1().write(|w| w.usic2().set_bit()),
            PeripheralClock::Pports => scu.cgatclr1().write(|w| w.pports().set_bit()),
            PeripheralClock::Wdt => scu.cgatclr2().write(|w| w.wdt().set_bit()),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Eth0 => scu.cgatclr2().write(|w| w.eth0().set_bit()),
            PeripheralClock::Dma0 => scu.cgatclr2().write(|w| w.dma0().set_bit()),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Dma1 => scu.cgatclr2().write(|w| w.dma1().set_bit()),
            PeripheralClock::Fce => scu.cgatclr2().write(|w| w.fce().set_bit()),
            PeripheralClock::Usb0 => scu.cgatclr2().write(|w| w.usb().set_bit()),
            #[cfg(any(feature = "xmc4300", feature = "xmc4800"))]
            PeripheralClock::Ecat0 => scu.cgatclr2().write(|w| w.ecat0().set_bit()),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ebu => scu.cgatclr3().write(|w| w.ebu().set_bit()),
        };
    }

    #[cfg(not(feature = "xmc4500"))]
    pub fn is_peripheral_clock_gated(&self, clock: PeripheralClock) -> bool {
        let scu = unsafe { &*SCU_CLK::ptr() };

        match clock {
            PeripheralClock::Vadc => scu.cgatstat0().read().vadc().bit_is_clear(),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Dsd => scu.cgatstat0().read().dsd().bit_is_clear(),
            PeripheralClock::Ccu40 => scu.cgatstat0().read().ccu40().bit_is_clear(),
            PeripheralClock::Ccu41 => scu.cgatstat0().read().ccu41().bit_is_clear(),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ccu42 => scu.cgatstat0().read().ccu42().bit_is_clear(),
            PeripheralClock::Ccu80 => scu.cgatstat0().read().ccu80().bit_is_clear(),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ccu81 => scu.cgatstat0().read().ccu81().bit_is_clear(),
            PeripheralClock::Posif0 => scu.cgatstat0().read().posif0().bit_is_clear(),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Posif1 => scu.cgatstat0().read().posif1().bit_is_clear(),
            PeripheralClock::Usic0 => scu.cgatstat0().read().usic0().bit_is_clear(),
            PeripheralClock::Eru1 => scu.cgatstat0().read().eru1().bit_is_clear(),
            PeripheralClock::Hrpwm0 => unimplemented!(), // Appears to be missing from SVD
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ccu43 => scu.cgatstat1().read().ccu43().bit_is_clear(),
            PeripheralClock::Ledtscu0 => scu.cgatstat1().read().ledtscu0().bit_is_clear(),
            PeripheralClock::Mcan => scu.cgatstat1().read().mcan0().bit_is_clear(),
            PeripheralClock::Dac => scu.cgatstat1().read().dac().bit_is_clear(),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Mmci => scu.cgatstat1().read().mmci().bit_is_clear(),
            PeripheralClock::Usic1 => scu.cgatstat1().read().usic1().bit_is_clear(),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Usic2 => scu.cgatstat1().read().usic2().bit_is_clear(),
            PeripheralClock::Pports => scu.cgatstat1().read().pports().bit_is_clear(),
            PeripheralClock::Wdt => scu.cgatstat2().read().wdt().bit_is_clear(),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Eth0 => scu.cgatstat2().read().eth0().bit_is_clear(),
            PeripheralClock::Dma0 => scu.cgatstat2().read().dma0().bit_is_clear(),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Dma1 => scu.cgatstat2().read().dma1().bit_is_clear(),
            PeripheralClock::Fce => scu.cgatstat2().read().fce().bit_is_clear(),
            PeripheralClock::Usb0 => scu.cgatstat2().read().usb().bit_is_clear(),
            #[cfg(any(feature = "xmc4300", feature = "xmc4800"))]
            PeripheralClock::Ecat0 => scu.cgatstat2().read().ecat0().bit_is_clear(),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ebu => scu.cgatstat3().read().ebu().bit_is_clear(),
        }
    }
    pub fn assert_peripheral_reset(&self, peripheral: PeripheralReset) {
        match peripheral {
            PeripheralReset::Wdt => {
                let reset = unsafe { &*SCU_RESET::ptr() };
                reset.prset2().write(|w| w.wdtrs().set_bit());
            }
            _ => unimplemented!(),
        };
    }

    pub fn deassert_peripheral_reset(&self, peripheral: PeripheralReset) {
        match peripheral {
            PeripheralReset::Wdt => {
                let scu = unsafe { &*SCU_RESET::ptr() };
                scu.prclr2().write(|w| w.wdtrs().set_bit());
            }
            _ => unimplemented!(),
        };
    }

    fn calibrate_temperature_sensor(&self, _offset: u32, _gain: u32) {
        unimplemented!();
    }

    fn enable_temperature_sensor(&self) {
        unimplemented!();
    }

    fn disable_temperature_sensor(&self) {
        unimplemented!();
    }

    fn is_temperature_sensor_enabled(&self) -> bool {
        unimplemented!();
    }

    fn start_temperature_measurement(&self) {
        unimplemented!();
    }

    fn get_temperature_measurement(&self) {
        unimplemented!()
    }

    fn is_temperature_sensor_busy(&self) -> bool {
        unimplemented!();
    }

    fn high_temperature(&self) -> bool {
        unimplemented!();
    }

    fn set_raw_temp_limits(&self, _lower: u32, _upper: u32) {
        unimplemented!();
    }

    fn low_temperature(&self) -> bool {
        unimplemented!();
    }

    pub fn write_to_retention_memory(&self, address: u32, data: u32) {
        // TODO: Clean this up
        let rmacr = ((address << 16) & 0xF0000) | 1;
        let general = unsafe { &*SCU_GENERAL::ptr() };

        general.rmdata().write(|w| unsafe { w.data().bits(data) });
        general.rmacr().write(|w| unsafe { w.bits(rmacr) });

        while general.mirrsts().read().rmx().bit_is_set() {}
    }

    pub fn read_from_retention_memory(&self, address: u32) -> u32 {
        let rmacr = (address << 16) & 0xF0000;
        let general = unsafe { &*SCU_GENERAL::ptr() };

        general.rmacr().write(|w| unsafe { w.bits(rmacr) });
        while general.mirrsts().read().rmx().bit_is_set() {}
        general.rmdata().read().data().bits()
    }

    fn clock_init() {
        unimplemented!();
    }

    pub fn trap_enable(&self, trap: u32) {
        let scu = unsafe { &*SCU_TRAP::ptr() };

        scu.trapdis()
            .modify(|r, w| unsafe { w.bits(r.bits() & !trap) });
    }

    pub fn trap_disable(&self, trap: u32) {
        let scu = unsafe { &*SCU_TRAP::ptr() };

        scu.trapdis()
            .modify(|r, w| unsafe { w.bits(r.bits() | trap) });
    }

    pub fn trap_get_status(&self) -> u32 {
        let scu = unsafe { &*SCU_TRAP::ptr() };
        scu.trapraw().read().bits()
    }

    pub fn trap_trigger(&self, trap: u32) {
        let scu = unsafe { &*SCU_TRAP::ptr() };
        scu.trapset().write(|w| unsafe { w.bits(trap) });
    }

    pub fn trap_clear_status(&self, trap: u32) {
        let scu = unsafe { &*SCU_TRAP::ptr() };
        scu.trapclr().write(|w| unsafe { w.bits(trap) });
    }
}
