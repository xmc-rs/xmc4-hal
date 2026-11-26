use crate::pac::{
    SCU_CLK, SCU_GENERAL, SCU_HIBERNATE, SCU_INTERRUPT, SCU_OSC, SCU_PARITY, SCU_PLL, SCU_POWER,
    SCU_RESET, SCU_TRAP,
};
use crate::time::Hertz;
use crate::Sealed;

/// Backup oscillator on every XMC4000 device.
pub const OFI_HZ: u32 = 24_000_000;

/// PDIV register value for an 8 MHz crystal (divider is this value plus one).
const PLL_PDIV_XTAL_8MHZ: u8 = 1;

/// NDIV register value for an 8 MHz crystal (divider is this value plus one).
const PLL_NDIV_XTAL_8MHZ: u8 = 89;

/// K2DIV register value for an 8 MHz crystal (divider is this value plus one).
const PLL_K2DIV_XTAL_8MHZ: u8 = 2;

/// Owned system-control block.
///
/// Construct it with [`ScuExt::constrain`] so the PAC clock peripheral is consumed.
/// [`Scu::new`] steals the same blocks for callers that do not hold them yet.
pub struct Scu {
    _clk: SCU_CLK,
    _pll: SCU_PLL,
    _osc: SCU_OSC,
    _general: SCU_GENERAL,
    _interrupt: SCU_INTERRUPT,
    _power: SCU_POWER,
    _reset: SCU_RESET,
    _trap: SCU_TRAP,
    _hibernate: SCU_HIBERNATE,
    _parity: SCU_PARITY,
    cfgr: Cfgr,
}

/// Extension trait implemented on the PAC clock block.
pub trait ScuExt: Sealed {
    /// Take ownership of the SCU and expose the clock configurator.
    fn constrain(self) -> Scu;
}

impl Sealed for SCU_CLK {}

impl ScuExt for SCU_CLK {
    fn constrain(self) -> Scu {
        Scu::from_clk(self)
    }
}

/// Frozen clock tree produced by [`Cfgr::freeze`].
#[derive(Copy, Clone, Debug)]
pub struct Clocks {
    sysclk: Hertz,
    cpu: Hertz,
    pclk: Hertz,
    ccu: Hertz,
}

impl Clocks {
    /// System clock after the PLL (or OFI) and the system divider.
    pub fn sysclk(&self) -> Hertz {
        self.sysclk
    }

    /// CPU clock.
    pub fn cpu(&self) -> Hertz {
        self.cpu
    }

    /// Peripheral-bus clock.
    pub fn pclk(&self) -> Hertz {
        self.pclk
    }

    /// CCU clock.
    pub fn ccu(&self) -> Hertz {
        self.ccu
    }
}

/// Clock configurator programmed by [`Cfgr::freeze`].
#[derive(Clone, Debug)]
pub struct Cfgr {
    /// External high-precision oscillator frequency. Zero keeps the internal 24 MHz oscillator.
    osc_hz: u32,
    use_pll: bool,
    pdiv: u8,
    ndiv: u8,
    k2div: u8,
    sys_div: u8,
    cpu_div: u8,
    pb_div: u8,
    ccu_div: u8,
}

impl Default for Cfgr {
    fn default() -> Self {
        Self {
            osc_hz: 0,
            use_pll: false,
            pdiv: PLL_PDIV_XTAL_8MHZ,
            ndiv: PLL_NDIV_XTAL_8MHZ,
            k2div: PLL_K2DIV_XTAL_8MHZ,
            sys_div: 1,
            cpu_div: 1,
            pb_div: 1,
            ccu_div: 1,
        }
    }
}

impl Cfgr {
    /// Run the system PLL from an 8 MHz crystal using the on-chip 120 MHz settings.
    pub fn pll_8mhz(&mut self) -> &mut Self {
        self.osc_hz = 8_000_000;
        self.use_pll = true;
        self.pdiv = PLL_PDIV_XTAL_8MHZ;
        self.ndiv = PLL_NDIV_XTAL_8MHZ;
        self.k2div = PLL_K2DIV_XTAL_8MHZ;
        self
    }

    /// Use the external oscillator at `hz` with explicit PLL register dividers (value written is divider minus one).
    pub fn pll(&mut self, hz: Hertz, pdiv: u8, ndiv: u8, k2div: u8) -> &mut Self {
        self.osc_hz = hz.0;
        self.use_pll = true;
        self.pdiv = pdiv;
        self.ndiv = ndiv;
        self.k2div = k2div;
        self
    }

    /// Stay on the 24 MHz backup oscillator.
    pub fn ofi(&mut self) -> &mut Self {
        self.use_pll = false;
        self.osc_hz = 0;
        self
    }

    /// System, CPU, peripheral, and CCU dividers. Each value is the real divider (1 or 2 for CPU, peripheral, and CCU).
    pub fn dividers(&mut self, sys: u8, cpu: u8, peripheral: u8, ccu: u8) -> &mut Self {
        self.sys_div = sys.max(1);
        self.cpu_div = cpu.max(1);
        self.pb_div = peripheral.max(1);
        self.ccu_div = ccu.max(1);
        self
    }

    /// Program the oscillators, PLL, and dividers, then return the resulting frequencies.
    pub fn freeze(&mut self) -> Clocks {
        let clk = unsafe { &*SCU_CLK::ptr() };
        let pll = unsafe { &*SCU_PLL::ptr() };
        let osc = unsafe { &*SCU_OSC::ptr() };

        // Always start from the backup clock so the PLL can be reconfigured.
        clk.sysclkcr().modify(|_, w| w.syssel().clear_bit());

        let parent = if self.use_pll && self.osc_hz > 0 {
            let oscval = (self.osc_hz / 2_500_000).saturating_sub(1) as u8;
            pll.pllcon0().modify(|_, w| w.pllpwd().clear_bit());
            osc.oschpctrl().modify(|_, w| unsafe {
                #[cfg(not(feature = "xmc4300"))]
                w.mode().value1();
                #[cfg(feature = "xmc4300")]
                w.mode().const_00();
                w.oscval().bits(oscval)
            });
            pll.pllcon0().modify(|_, w| w.oscres().clear_bit());
            while !(pll.pllstat().read().pllhv().bit_is_set()
                && pll.pllstat().read().plllv().bit_is_set()
                && pll.pllstat().read().pllsp().bit_is_set())
            {}

            pll.pllcon0()
                .modify(|_, w| w.vcopwd().clear_bit().pllpwd().clear_bit());
            start_system_pll(pll, self.pdiv, self.ndiv, self.k2div, self.osc_hz);
            let vco = self.osc_hz / (self.pdiv as u32 + 1) * (self.ndiv as u32 + 1);
            vco / (self.k2div as u32 + 1)
        } else {
            OFI_HZ
        };

        clk.sysclkcr()
            .modify(|_, w| unsafe { w.sysdiv().bits(self.sys_div.saturating_sub(1)) });
        clk.cpuclkcr().modify(|_, w| {
            if self.cpu_div > 1 {
                w.cpudiv().set_bit()
            } else {
                w.cpudiv().clear_bit()
            }
        });
        clk.pbclkcr().modify(|_, w| {
            if self.pb_div > 1 {
                w.pbdiv().set_bit()
            } else {
                w.pbdiv().clear_bit()
            }
        });
        clk.ccuclkcr().modify(|_, w| {
            if self.ccu_div > 1 {
                w.ccudiv().set_bit()
            } else {
                w.ccudiv().clear_bit()
            }
        });

        if self.use_pll {
            clk.sysclkcr().modify(|_, w| w.syssel().set_bit());
        }

        let sysclk = parent / self.sys_div as u32;
        Clocks {
            sysclk: Hertz(sysclk),
            cpu: Hertz(sysclk / self.cpu_div as u32),
            pclk: Hertz(sysclk / self.pb_div as u32),
            ccu: Hertz(sysclk / self.ccu_div as u32),
        }
    }
}

fn start_system_pll(
    pll: &crate::pac::scu_pll::RegisterBlock,
    pdiv: u8,
    ndiv: u8,
    k2div: u8,
    osc_hz: u32,
) {
    // Prescaler mode, then lock, then ramp K2 down toward the requested divider.
    pll.pllcon0().modify(|_, w| w.vcobyp().set_bit());
    while pll.pllstat().read().vcobyst().bit_is_clear() {}
    pll.pllcon0().modify(|_, w| w.findis().set_bit());

    let vco_mhz = (osc_hz / 1_000_000) * (ndiv as u32 + 1) / (pdiv as u32 + 1);
    let mut k_step = (vco_mhz / (OFI_HZ / 1_000_000)).max(1) as u8;
    pll.pllcon1().modify(|_, w| unsafe {
        w.pdiv().bits(pdiv);
        w.ndiv().bits(ndiv);
        w.k2div().bits(k_step.saturating_sub(1))
    });
    pll.pllcon0().modify(|_, w| w.oscdiscdis().set_bit());
    pll.pllcon0().modify(|_, w| w.findis().clear_bit());
    pll.pllcon0().modify(|_, w| w.resld().set_bit());
    while pll.pllstat().read().vcolock().bit_is_clear() {}
    pll.pllcon0().modify(|_, w| w.vcobyp().clear_bit());
    while pll.pllstat().read().vcobyst().bit_is_set() {}

    let target = k2div.saturating_add(1);
    for threshold in [60u32, 90, vco_mhz.max(1)] {
        let next = (vco_mhz / threshold).max(1) as u8;
        if target < k_step && next < k_step {
            k_step = next.max(target);
            pll.pllcon1()
                .modify(|_, w| unsafe { w.k2div().bits(k_step.saturating_sub(1)) });
            while pll.pllstat().read().k2rdy().bit_is_clear() {}
        }
    }
    if k_step != target {
        pll.pllcon1()
            .modify(|_, w| unsafe { w.k2div().bits(k2div) });
        while pll.pllstat().read().k2rdy().bit_is_clear() {}
    }
}

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
    #[cfg(any(
        feature = "xmc4400",
        feature = "xmc4500",
        feature = "xmc4700",
        feature = "xmc4800"
    ))]
    Dsd,
    Ccu40,
    Ccu41,
    #[cfg(any(
        feature = "xmc4400",
        feature = "xmc4500",
        feature = "xmc4700",
        feature = "xmc4800"
    ))]
    Ccu42,
    Ccu80,
    #[cfg(any(
        feature = "xmc4400",
        feature = "xmc4500",
        feature = "xmc4700",
        feature = "xmc4800"
    ))]
    Ccu81,
    #[cfg(not(feature = "xmc4300"))]
    Posif0,
    #[cfg(any(
        feature = "xmc4400",
        feature = "xmc4500",
        feature = "xmc4700",
        feature = "xmc4800"
    ))]
    Posif1,
    Usic0,
    Eru1,
    #[cfg(any(feature = "xmc4100", feature = "xmc4200", feature = "xmc4400"))]
    Hrpwm0,
    #[cfg(any(
        feature = "xmc4400",
        feature = "xmc4500",
        feature = "xmc4700",
        feature = "xmc4800"
    ))]
    Ccu43,
    Ledts0,
    Mcan,
    Dac,
    #[cfg(feature = "sdmmc")]
    Sdmmc,
    Usic1,
    #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
    Usic2,
    Ports,
    Wdt,
    #[cfg(not(any(feature = "xmc4100", feature = "xmc4200")))]
    Eth0,
    Gpdma0,
    #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
    Gpdma1,
    Fce,
    Usb0,
    #[cfg(feature = "ecat")]
    Ecat0,
    #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
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
    #[cfg(any(feature = "xmc4100", feature = "xmc4200", feature = "xmc4400"))]
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

impl TryFrom<u8> for BootMode {
    type Error = Status;

    fn try_from(bits: u8) -> Result<BootMode, Status> {
        match bits {
            0 => Ok(BootMode::Normal),
            1 => Ok(BootMode::AscBsl),
            2 => Ok(BootMode::Bmi),
            3 => Ok(BootMode::CanBsl),
            4 => Ok(BootMode::PsramBoot),
            5 => Ok(BootMode::Abm0),
            6 => Ok(BootMode::Abm1),
            7 => Ok(BootMode::Fabm),
            _ => Err(Status::Error),
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

impl InterruptEvent {
    /// Bit mask in `SRMSK` / `SRRAW` / `SRSET` / `SRCLR`.
    fn mask(self) -> u32 {
        1u32 << (self as u32)
    }
}

impl TryFrom<u32> for InterruptEvent {
    type Error = Status;

    fn try_from(bits: u32) -> Result<InterruptEvent, Status> {
        match bits {
            0 => Ok(InterruptEvent::WdtPreWarn),
            1 => Ok(InterruptEvent::RtcPeriodic),
            2 => Ok(InterruptEvent::RtcAlarm),
            3 => Ok(InterruptEvent::DlrRequestOverrun),
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4700"))]
            6 => Ok(InterruptEvent::Lpaclr),
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            7 => Ok(InterruptEvent::Lpacth0),
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            8 => Ok(InterruptEvent::Lpacth1),
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            9 => Ok(InterruptEvent::Lpacst),
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            10 => Ok(InterruptEvent::Lpacclr),
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            11 => Ok(InterruptEvent::Lpacset),
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            12 => Ok(InterruptEvent::Hintst),
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            13 => Ok(InterruptEvent::Hintclr),
            #[cfg(not(feature = "xmc4500"))]
            #[cfg(not(feature = "xmc4300"))]
            #[cfg(not(feature = "xmc4700"))]
            14 => Ok(InterruptEvent::Hintset),
            17 => Ok(InterruptEvent::Hdcrclr),
            18 => Ok(InterruptEvent::Hdcrset),
            19 => Ok(InterruptEvent::Hdcr),
            21 => Ok(InterruptEvent::Oscsictrl),
            23 => Ok(InterruptEvent::Osculctrl),
            24 => Ok(InterruptEvent::RtcCtr),
            25 => Ok(InterruptEvent::RtcAtim0),
            26 => Ok(InterruptEvent::RtcAtim1),
            27 => Ok(InterruptEvent::RtcTim0),
            28 => Ok(InterruptEvent::RtcTim1),
            29 => Ok(InterruptEvent::RetentionMemory),
            _ => Err(Status::Error),
        }
    }
}

impl Scu {
    /// Steal every SCU block. Prefer [`ScuExt::constrain`] when `SCU_CLK` is still owned.
    pub fn new() -> Self {
        unsafe { Self::from_clk(SCU_CLK::steal()) }
    }

    fn from_clk(clk: SCU_CLK) -> Self {
        unsafe {
            Self {
                _clk: clk,
                _pll: SCU_PLL::steal(),
                _osc: SCU_OSC::steal(),
                _general: SCU_GENERAL::steal(),
                _interrupt: SCU_INTERRUPT::steal(),
                _power: SCU_POWER::steal(),
                _reset: SCU_RESET::steal(),
                _trap: SCU_TRAP::steal(),
                _hibernate: SCU_HIBERNATE::steal(),
                _parity: SCU_PARITY::steal(),
                cfgr: Cfgr::default(),
            }
        }
    }

    /// Clock configurator. [`Cfgr::freeze`] programs the PLL and dividers.
    pub fn cfgr(&mut self) -> &mut Cfgr {
        &mut self.cfgr
    }

    pub fn enable_event(&self, event: InterruptEvent) {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };

        scu.srmsk()
            .modify(|r, w| unsafe { w.bits(r.bits() | event.mask()) });
    }

    pub fn disable_event(&self, event: InterruptEvent) {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };

        scu.srmsk()
            .modify(|r, w| unsafe { w.bits(r.bits() & !event.mask()) });
    }

    pub fn trigger_event(&self, event: InterruptEvent) {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };
        scu.srset().write(|w| unsafe { w.bits(event.mask()) });
    }

    /// Raw service-request status bits.
    pub fn event_status_raw(&self) -> u32 {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };
        scu.srraw().read().bits()
    }

    /// `true` when `event` is pending in `SRRAW`.
    pub fn is_event_pending(&self, event: InterruptEvent) -> bool {
        self.event_status_raw() & event.mask() != 0
    }

    pub fn clear_event_status(&self, event: InterruptEvent) {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };
        scu.srclr().write(|w| unsafe { w.bits(event.mask()) });
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

    pub fn read_gpr(&self, index: u32) -> Option<u32> {
        let scu = unsafe { &*SCU_GENERAL::ptr() };
        match index {
            0 => Some(scu.gpr0().read().bits()),
            1 => Some(scu.gpr1().read().bits()),
            _ => None,
        }
    }

    pub fn write_gpr(&self, index: u32, data: u32) -> Result<(), Status> {
        let scu = unsafe { &*SCU_GENERAL::ptr() };
        match index {
            0 => scu.gpr0().write(|w| unsafe { w.bits(data) }),
            1 => scu.gpr1().write(|w| unsafe { w.bits(data) }),
            _ => return Err(Status::Error),
        };
        Ok(())
    }

    pub fn enable_hibernate_domain(&self) {
        let scu = unsafe { &*SCU_POWER::ptr() };
        if scu.pwrstat().read().hiben().bit_is_clear() {
            scu.pwrset().write(|w| w.hib().set_bit());
            while scu.pwrstat().read().hiben().bit_is_clear() {}
        }
        let reset = unsafe { &*SCU_RESET::ptr() };
        if reset.rststat().read().hibrs().bit_is_set() {
            reset.rstclr().write(|w| w.hibrs().set_bit());
            while reset.rststat().read().hibrs().bit_is_set() {}
        }
    }
    pub fn is_hibernate_domain_enabled(&self) -> bool {
        let power = unsafe { &*SCU_POWER::ptr() };
        let reset = unsafe { &*SCU_RESET::ptr() };

        power.pwrstat().read().hiben().bit_is_set() && reset.rststat().read().hibrs().bit_is_clear()
    }

    /// Access the currently programmed boot mode.
    pub fn get_boot_mode(&self) -> Option<BootMode> {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        BootMode::try_from(general.stcon().read().swcon().bits()).ok()
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
            #[cfg(any(feature = "xmc4100", feature = "xmc4200", feature = "xmc4400"))]
            PeripheralClock::Hrpwm0 => scu.cgatset0().write(|w| w.hrpwm0().set_bit()),
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
            #[cfg(any(feature = "xmc4100", feature = "xmc4200", feature = "xmc4400"))]
            PeripheralClock::Hrpwm0 => scu.cgatclr0().write(|w| w.hrpwm0().set_bit()),
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
            PeripheralClock::Vadc => scu.cgatstat0().read().vadc().bit_is_set(),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Dsd => scu.cgatstat0().read().dsd().bit_is_set(),
            PeripheralClock::Ccu40 => scu.cgatstat0().read().ccu40().bit_is_set(),
            PeripheralClock::Ccu41 => scu.cgatstat0().read().ccu41().bit_is_set(),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ccu42 => scu.cgatstat0().read().ccu42().bit_is_set(),
            PeripheralClock::Ccu80 => scu.cgatstat0().read().ccu80().bit_is_set(),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ccu81 => scu.cgatstat0().read().ccu81().bit_is_set(),
            PeripheralClock::Posif0 => scu.cgatstat0().read().posif0().bit_is_set(),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Posif1 => scu.cgatstat0().read().posif1().bit_is_set(),
            PeripheralClock::Usic0 => scu.cgatstat0().read().usic0().bit_is_set(),
            PeripheralClock::Eru1 => scu.cgatstat0().read().eru1().bit_is_set(),
            #[cfg(any(feature = "xmc4100", feature = "xmc4200", feature = "xmc4400"))]
            PeripheralClock::Hrpwm0 => scu.cgatstat0().read().hrpwm0().bit_is_set(),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ccu43 => scu.cgatstat1().read().ccu43().bit_is_set(),
            PeripheralClock::Ledtscu0 => scu.cgatstat1().read().ledtscu0().bit_is_set(),
            PeripheralClock::Mcan => scu.cgatstat1().read().mcan0().bit_is_set(),
            PeripheralClock::Dac => scu.cgatstat1().read().dac().bit_is_set(),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Mmci => scu.cgatstat1().read().mmci().bit_is_set(),
            PeripheralClock::Usic1 => scu.cgatstat1().read().usic1().bit_is_set(),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Usic2 => scu.cgatstat1().read().usic2().bit_is_set(),
            PeripheralClock::Pports => scu.cgatstat1().read().pports().bit_is_set(),
            PeripheralClock::Wdt => scu.cgatstat2().read().wdt().bit_is_set(),
            #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Eth0 => scu.cgatstat2().read().eth0().bit_is_set(),
            PeripheralClock::Dma0 => scu.cgatstat2().read().dma0().bit_is_set(),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Dma1 => scu.cgatstat2().read().dma1().bit_is_set(),
            PeripheralClock::Fce => scu.cgatstat2().read().fce().bit_is_set(),
            PeripheralClock::Usb0 => scu.cgatstat2().read().usb().bit_is_set(),
            #[cfg(any(feature = "xmc4300", feature = "xmc4800"))]
            PeripheralClock::Ecat0 => scu.cgatstat2().read().ecat0().bit_is_set(),
            #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
            PeripheralClock::Ebu => scu.cgatstat3().read().ebu().bit_is_set(),
        }
    }
    pub fn assert_peripheral_reset(&self, peripheral: PeripheralReset) {
        let reset = unsafe { &*SCU_RESET::ptr() };
        match peripheral {
            PeripheralReset::Vadc => reset.prset0().write(|w| w.vadcrs().set_bit()),
            #[cfg(any(
                feature = "xmc4400",
                feature = "xmc4500",
                feature = "xmc4700",
                feature = "xmc4800"
            ))]
            PeripheralReset::Dsd => reset.prset0().write(|w| w.dsdrs().set_bit()),
            PeripheralReset::Ccu40 => reset.prset0().write(|w| w.ccu40rs().set_bit()),
            PeripheralReset::Ccu41 => reset.prset0().write(|w| w.ccu41rs().set_bit()),
            #[cfg(any(
                feature = "xmc4400",
                feature = "xmc4500",
                feature = "xmc4700",
                feature = "xmc4800"
            ))]
            PeripheralReset::Ccu42 => reset.prset0().write(|w| w.ccu42rs().set_bit()),
            PeripheralReset::Ccu80 => reset.prset0().write(|w| w.ccu80rs().set_bit()),
            #[cfg(any(
                feature = "xmc4400",
                feature = "xmc4500",
                feature = "xmc4700",
                feature = "xmc4800"
            ))]
            PeripheralReset::Ccu81 => reset.prset0().write(|w| w.ccu81rs().set_bit()),
            #[cfg(not(feature = "xmc4300"))]
            PeripheralReset::Posif0 => reset.prset0().write(|w| w.posif0rs().set_bit()),
            #[cfg(any(
                feature = "xmc4400",
                feature = "xmc4500",
                feature = "xmc4700",
                feature = "xmc4800"
            ))]
            PeripheralReset::Posif1 => reset.prset0().write(|w| w.posif1rs().set_bit()),
            PeripheralReset::Usic0 => reset.prset0().write(|w| w.usic0rs().set_bit()),
            PeripheralReset::Eru1 => reset.prset0().write(|w| w.eru1rs().set_bit()),
            #[cfg(any(feature = "xmc4100", feature = "xmc4200", feature = "xmc4400"))]
            PeripheralReset::Hrpwm0 => reset.prset0().write(|w| w.hrpwm0rs().set_bit()),
            #[cfg(any(
                feature = "xmc4400",
                feature = "xmc4500",
                feature = "xmc4700",
                feature = "xmc4800"
            ))]
            PeripheralReset::Ccu43 => reset.prset1().write(|w| w.ccu43rs().set_bit()),
            PeripheralReset::Ledts0 => reset.prset1().write(|w| w.ledtscu0rs().set_bit()),
            PeripheralReset::Mcan => reset.prset1().write(|w| w.mcan0rs().set_bit()),
            PeripheralReset::Dac => reset.prset1().write(|w| w.dacrs().set_bit()),
            #[cfg(feature = "sdmmc")]
            PeripheralReset::Sdmmc => reset.prset1().write(|w| w.mmcirs().set_bit()),
            PeripheralReset::Usic1 => reset.prset1().write(|w| w.usic1rs().set_bit()),
            #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralReset::Usic2 => reset.prset1().write(|w| w.usic2rs().set_bit()),
            PeripheralReset::Ports => reset.prset1().write(|w| w.pportsrs().set_bit()),
            PeripheralReset::Wdt => reset.prset2().write(|w| w.wdtrs().set_bit()),
            #[cfg(not(any(feature = "xmc4100", feature = "xmc4200")))]
            PeripheralReset::Eth0 => reset.prset2().write(|w| w.eth0rs().set_bit()),
            PeripheralReset::Gpdma0 => reset.prset2().write(|w| w.dma0rs().set_bit()),
            #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralReset::Gpdma1 => reset.prset2().write(|w| w.dma1rs().set_bit()),
            PeripheralReset::Fce => reset.prset2().write(|w| w.fcers().set_bit()),
            PeripheralReset::Usb0 => reset.prset2().write(|w| w.usbrs().set_bit()),
            #[cfg(feature = "ecat")]
            PeripheralReset::Ecat0 => reset.prset2().write(|w| w.ecat0rs().set_bit()),
            #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralReset::Ebu => reset.prset3().write(|w| w.eburs().set_bit()),
        };
    }

    pub fn deassert_peripheral_reset(&self, peripheral: PeripheralReset) {
        let reset = unsafe { &*SCU_RESET::ptr() };
        match peripheral {
            PeripheralReset::Vadc => reset.prclr0().write(|w| w.vadcrs().set_bit()),
            #[cfg(any(
                feature = "xmc4400",
                feature = "xmc4500",
                feature = "xmc4700",
                feature = "xmc4800"
            ))]
            PeripheralReset::Dsd => reset.prclr0().write(|w| w.dsdrs().set_bit()),
            PeripheralReset::Ccu40 => reset.prclr0().write(|w| w.ccu40rs().set_bit()),
            PeripheralReset::Ccu41 => reset.prclr0().write(|w| w.ccu41rs().set_bit()),
            #[cfg(any(
                feature = "xmc4400",
                feature = "xmc4500",
                feature = "xmc4700",
                feature = "xmc4800"
            ))]
            PeripheralReset::Ccu42 => reset.prclr0().write(|w| w.ccu42rs().set_bit()),
            PeripheralReset::Ccu80 => reset.prclr0().write(|w| w.ccu80rs().set_bit()),
            #[cfg(any(
                feature = "xmc4400",
                feature = "xmc4500",
                feature = "xmc4700",
                feature = "xmc4800"
            ))]
            PeripheralReset::Ccu81 => reset.prclr0().write(|w| w.ccu81rs().set_bit()),
            #[cfg(not(feature = "xmc4300"))]
            PeripheralReset::Posif0 => reset.prclr0().write(|w| w.posif0rs().set_bit()),
            #[cfg(any(
                feature = "xmc4400",
                feature = "xmc4500",
                feature = "xmc4700",
                feature = "xmc4800"
            ))]
            PeripheralReset::Posif1 => reset.prclr0().write(|w| w.posif1rs().set_bit()),
            PeripheralReset::Usic0 => reset.prclr0().write(|w| w.usic0rs().set_bit()),
            PeripheralReset::Eru1 => reset.prclr0().write(|w| w.eru1rs().set_bit()),
            #[cfg(any(feature = "xmc4100", feature = "xmc4200", feature = "xmc4400"))]
            PeripheralReset::Hrpwm0 => reset.prclr0().write(|w| w.hrpwm0rs().set_bit()),
            #[cfg(any(
                feature = "xmc4400",
                feature = "xmc4500",
                feature = "xmc4700",
                feature = "xmc4800"
            ))]
            PeripheralReset::Ccu43 => reset.prclr1().write(|w| w.ccu43rs().set_bit()),
            PeripheralReset::Ledts0 => reset.prclr1().write(|w| w.ledtscu0rs().set_bit()),
            PeripheralReset::Mcan => reset.prclr1().write(|w| w.mcan0rs().set_bit()),
            PeripheralReset::Dac => reset.prclr1().write(|w| w.dacrs().set_bit()),
            #[cfg(feature = "sdmmc")]
            PeripheralReset::Sdmmc => reset.prclr1().write(|w| w.mmcirs().set_bit()),
            PeripheralReset::Usic1 => reset.prclr1().write(|w| w.usic1rs().set_bit()),
            #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralReset::Usic2 => reset.prclr1().write(|w| w.usic2rs().set_bit()),
            PeripheralReset::Ports => reset.prclr1().write(|w| w.pportsrs().set_bit()),
            PeripheralReset::Wdt => reset.prclr2().write(|w| w.wdtrs().set_bit()),
            #[cfg(not(any(feature = "xmc4100", feature = "xmc4200")))]
            PeripheralReset::Eth0 => reset.prclr2().write(|w| w.eth0rs().set_bit()),
            PeripheralReset::Gpdma0 => reset.prclr2().write(|w| w.dma0rs().set_bit()),
            #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralReset::Gpdma1 => reset.prclr2().write(|w| w.dma1rs().set_bit()),
            PeripheralReset::Fce => reset.prclr2().write(|w| w.fcers().set_bit()),
            PeripheralReset::Usb0 => reset.prclr2().write(|w| w.usbrs().set_bit()),
            #[cfg(feature = "ecat")]
            PeripheralReset::Ecat0 => reset.prclr2().write(|w| w.ecat0rs().set_bit()),
            #[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
            PeripheralReset::Ebu => reset.prclr3().write(|w| w.eburs().set_bit()),
        };
    }

    /// Write the die-temperature calibration. Reference and bandgap trims match XMCLib.
    pub fn calibrate_temperature_sensor(&self, offset: u8, gain: u8) {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        general.dtscon().modify(|_, w| unsafe {
            w.offset().bits(offset);
            w.gain().bits(gain);
            w.reftrim().bits(0x4);
            w.bgtrim().bits(0x8)
        });
    }

    pub fn enable_temperature_sensor(&self) {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        general.dtscon().modify(|_, w| w.pwd().clear_bit());
    }

    pub fn disable_temperature_sensor(&self) {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        general.dtscon().modify(|_, w| w.pwd().set_bit());
    }

    pub fn is_temperature_sensor_enabled(&self) -> bool {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        general.dtscon().read().pwd().bit_is_clear()
    }

    pub fn start_temperature_measurement(&self) -> Status {
        if !self.is_temperature_sensor_enabled() {
            return Status::Error;
        }
        if self.is_temperature_sensor_busy() {
            return Status::Busy;
        }
        let general = unsafe { &*SCU_GENERAL::ptr() };
        general.dtscon().modify(|_, w| w.start().set_bit());
        Status::Ok
    }

    pub fn get_temperature_measurement(&self) -> Option<u16> {
        if !self.is_temperature_sensor_enabled() {
            return None;
        }
        let general = unsafe { &*SCU_GENERAL::ptr() };
        Some(general.dtsstat().read().result().bits())
    }

    pub fn is_temperature_sensor_busy(&self) -> bool {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        general.dtsstat().read().busy().bit_is_set()
    }

    pub fn is_temperature_sensor_ready(&self) -> bool {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        general.dtsstat().read().rdy().bit_is_set()
    }

    #[cfg(any(feature = "xmc4100", feature = "xmc4200", feature = "xmc4400"))]
    pub fn high_temperature(&self) -> bool {
        if !self.is_temperature_sensor_enabled() {
            return false;
        }
        let general = unsafe { &*SCU_GENERAL::ptr() };
        general.dtempalarm().read().overfl().bit_is_set()
    }

    #[cfg(any(feature = "xmc4100", feature = "xmc4200", feature = "xmc4400"))]
    pub fn low_temperature(&self) -> bool {
        if !self.is_temperature_sensor_enabled() {
            return false;
        }
        let general = unsafe { &*SCU_GENERAL::ptr() };
        general.dtempalarm().read().underfl().bit_is_set()
    }

    #[cfg(any(feature = "xmc4100", feature = "xmc4200", feature = "xmc4400"))]
    pub fn set_raw_temp_limits(&self, lower: u16, upper: u16) {
        self.enable_temperature_sensor();
        let general = unsafe { &*SCU_GENERAL::ptr() };
        general
            .dtemplim()
            .write(|w| unsafe { w.lower().bits(lower).upper().bits(upper) });
    }

    /// Enable an NMI request source.
    pub fn enable_nmi(&self, request: NmiReq) {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };
        scu.nmireqen().modify(|_, w| match request {
            NmiReq::WdtWarn => w.prwarn().set_bit(),
            NmiReq::RtcPi => w.pi().set_bit(),
            NmiReq::RtcAi => w.ai().set_bit(),
            NmiReq::Eru00 => w.eru00().set_bit(),
            NmiReq::Eru01 => w.eru01().set_bit(),
            NmiReq::Eru02 => w.eru02().set_bit(),
            NmiReq::Eru03 => w.eru03().set_bit(),
        });
    }

    /// Disable an NMI request source.
    pub fn disable_nmi(&self, request: NmiReq) {
        let scu = unsafe { &*SCU_INTERRUPT::ptr() };
        scu.nmireqen().modify(|_, w| match request {
            NmiReq::WdtWarn => w.prwarn().clear_bit(),
            NmiReq::RtcPi => w.pi().clear_bit(),
            NmiReq::RtcAi => w.ai().clear_bit(),
            NmiReq::Eru00 => w.eru00().clear_bit(),
            NmiReq::Eru01 => w.eru01().clear_bit(),
            NmiReq::Eru02 => w.eru02().clear_bit(),
            NmiReq::Eru03 => w.eru03().clear_bit(),
        });
    }

    /// Gate or release clocks while the CPU is in sleep. `true` keeps the clock running.
    pub fn configure_sleep(&self, usb: bool, ccu: bool, wdt: bool) {
        let clk = unsafe { &*SCU_CLK::ptr() };
        clk.sleepcr().modify(|_, w| {
            let w = if usb {
                w.usbcr().set_bit()
            } else {
                w.usbcr().clear_bit()
            };
            let w = if ccu {
                w.ccucr().set_bit()
            } else {
                w.ccucr().clear_bit()
            };
            if wdt {
                w.wdtcr().set_bit()
            } else {
                w.wdtcr().clear_bit()
            }
        });
    }

    /// Enter deep-sleep configuration: flash and PLL power-down plus the same gated clocks as sleep.
    pub fn configure_deep_sleep(
        &self,
        flash_power_down: bool,
        pll_power_down: bool,
        vco_power_down: bool,
    ) {
        let clk = unsafe { &*SCU_CLK::ptr() };
        clk.dsleepcr().modify(|_, w| {
            let w = if flash_power_down {
                w.fpdn().set_bit()
            } else {
                w.fpdn().clear_bit()
            };
            let w = if pll_power_down {
                w.pllpdn().set_bit()
            } else {
                w.pllpdn().clear_bit()
            };
            if vco_power_down {
                w.vcopdn().set_bit()
            } else {
                w.vcopdn().clear_bit()
            }
        });
    }

    /// Select the RTC clock in the hibernate domain and wait until the mirror write completes.
    pub fn set_rtc_clock_source(&self, source: RtcClockSource) {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        let hibernate = unsafe { &*SCU_HIBERNATE::ptr() };
        while general.mirrsts().read().hdcr().bit_is_set() {}
        hibernate.hdcr().modify(|_, w| match source {
            RtcClockSource::Osi => w.rcs().clear_bit(),
            RtcClockSource::Ulp => w.rcs().set_bit(),
        });
    }

    /// Select the standby clock and wait until the mirror write completes.
    pub fn set_standby_clock_source(&self, source: StdbyClockSource) {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        let hibernate = unsafe { &*SCU_HIBERNATE::ptr() };
        while general.mirrsts().read().hdcr().bit_is_set() {}
        hibernate.hdcr().modify(|_, w| match source {
            StdbyClockSource::Osi => w.stdbysel().clear_bit(),
            StdbyClockSource::Osculp => w.stdbysel().set_bit(),
        });
    }

    pub fn enable_parity(&self, memory: Parity) {
        self.parity_bit(memory, true);
    }

    pub fn disable_parity(&self, memory: Parity) {
        self.parity_bit(memory, false);
    }

    fn parity_bit(&self, memory: Parity, enable: bool) {
        let parity = unsafe { &*SCU_PARITY::ptr() };
        macro_rules! flag {
            ($field:ident) => {
                parity.peen().modify(|_, w| {
                    if enable {
                        w.$field().set_bit()
                    } else {
                        w.$field().clear_bit()
                    }
                });
            };
        }
        match memory {
            Parity::PsramMem => {
                flag!(peenps);
            }
            Parity::Dsram1Mem => {
                flag!(peends1);
            }
            Parity::Usic0Mem => {
                flag!(peenu0);
            }
            Parity::Usic1Mem => {
                flag!(peenu1);
            }
            Parity::McanMem => {
                flag!(peenmc);
            }
            Parity::PmuMem => {
                flag!(peenpprf);
            }
            Parity::UsbMem => {
                flag!(peenusb);
            }
            _ => {}
        }
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
