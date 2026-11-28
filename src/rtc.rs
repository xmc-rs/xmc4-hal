#![allow(dead_code)]

use core::convert::TryFrom;

use crate::pac::{RTC, SCU_GENERAL, SCU_POWER, SCU_RESET};
use crate::sealed::Sealed;
// use crate::scu::Scu;

/// Maximum number of seconds for the RTC time
const MAX_SECONDS: u8 = 59;
/// Maximum number of minutes for the RTC time
const MAX_MINUTES: u8 = 59;
/// Maximum number of hours for the RTC time
const MAX_HOURS: u8 = 23;
/// Maximum number of days for the RTC time
const MAX_DAYS: u8 = 31;
/// Maximum number of years for the RTC time
const MAX_YEAR: u16 = 65535;
/// Offset for year in standard time representation
const YEAR_OFFSET: u16 = 1900;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Seconds = 0x01,
    Minutes = 0x02,
    Hours = 0x04,
    Days = 0x08,
    Months = 0x20,
    Years = 0x40,
    Alarm = 0x100,
}

impl TryFrom<u32> for Event {
    type Error = u32;

    fn try_from(bits: u32) -> Result<Self, Self::Error> {
        match bits {
            0x01 => Ok(Event::Seconds),
            0x02 => Ok(Event::Minutes),
            0x04 => Ok(Event::Hours),
            0x08 => Ok(Event::Days),
            0x20 => Ok(Event::Months),
            0x40 => Ok(Event::Years),
            0x100 => Ok(Event::Alarm),
            other => Err(other),
        }
    }
}

impl From<Event> for u32 {
    fn from(bits: Event) -> Self {
        bits as u32
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Current month in RTC time
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Month::January),
            1 => Some(Month::February),
            2 => Some(Month::March),
            3 => Some(Month::April),
            4 => Some(Month::May),
            5 => Some(Month::June),
            6 => Some(Month::July),
            7 => Some(Month::August),
            8 => Some(Month::September),
            9 => Some(Month::October),
            10 => Some(Month::November),
            11 => Some(Month::December),
            _ => None,
        }
    }
}

impl From<u8> for Month {
    fn from(val: u8) -> Self {
        Month::from_u8(val).unwrap_or(Month::January)
    }
}

impl From<Month> for u8 {
    fn from(val: Month) -> Self {
        val as u8
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Current weekday in RTC time
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Weekday {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Weekday::Sunday),
            1 => Some(Weekday::Monday),
            2 => Some(Weekday::Tuesday),
            3 => Some(Weekday::Wednesday),
            4 => Some(Weekday::Thursday),
            5 => Some(Weekday::Friday),
            6 => Some(Weekday::Saturday),
            _ => None,
        }
    }
}

impl From<u8> for Weekday {
    fn from(val: u8) -> Self {
        Weekday::from_u8(val).unwrap_or(Weekday::Sunday)
    }
}

impl From<Weekday> for u8 {
    fn from(val: Weekday) -> Self {
        val as u8
    }
}

/// Representation of RTC time objects
pub struct Time {
    /// Number of seconds in RTC time
    pub second: u8,
    /// Number of minutes in RTC time
    pub minute: u8,
    /// Number of hours in RTC time
    pub hour: u8,
    /// Number of days in RTC time
    pub day: u8,
    /// Day of the week in RTC time
    pub weekday: Weekday,
    /// Month of the year in RTC time
    pub month: Month,
    /// Year in RTC time
    pub year: u16,
}

/// Alarm compare value. The alarm registers have no weekday field.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Alarm {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: Month,
    pub year: u16,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum WakeupEvent {
    Alarm,
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

pub trait RtcExt: Sealed {
    fn constrain(self) -> Rtc;
}

impl Sealed for RTC {}

impl RtcExt for RTC {
    fn constrain(self) -> Rtc {
        Rtc::new(self)
    }
}

#[derive(Debug)]
pub struct Rtc {
    pub regs: RTC,
}

impl Rtc {
    fn wait_for_rtc_ctr_ready() {
        let scu = unsafe { &*SCU_GENERAL::ptr() };
        while scu.mirrsts().read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
    }

    fn wait_for_rtc_msksr_ready() {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        while general.mirrsts().read().rtc_msksr().bit_is_set() {
            // Check SCU_MIRRSTS to ensure that the event mask register is accessible.
        }
    }

    fn wait_for_rtc_clrsr_ready() {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        while general.mirrsts().read().rtc_clrsr().bit_is_set() {
            // Check SCU_MIRRSTS to ensure that the event clear register is accessible.
        }
    }

    pub fn new(rtc: RTC) -> Self {
        Self { regs: rtc }
    }

    pub fn start(&self) {
        Self::wait_for_rtc_ctr_ready();
        self.regs.ctr().modify(|_, w| w.enb().set_bit());
    }

    pub fn stop(&self) {
        Self::wait_for_rtc_ctr_ready();
        self.regs.ctr().modify(|_, w| w.enb().clear_bit());
    }

    pub fn is_running(&self) -> bool {
        Self::wait_for_rtc_ctr_ready();
        self.regs.ctr().read().enb().bit_is_set()
    }

    pub fn set_prescaler(&self, prescaler: u16) {
        Self::wait_for_rtc_ctr_ready();
        self.regs
            .ctr()
            .modify(|_, w| unsafe { w.div().bits(prescaler) });
    }

    pub fn set_time(&self, time: Time) {
        assert!(time.second <= MAX_SECONDS);
        assert!(time.minute <= MAX_MINUTES);
        assert!(time.hour <= MAX_HOURS);
        assert!(time.day < MAX_DAYS);
        assert!(time.year < MAX_YEAR);

        Self::wait_for_rtc_ctr_ready();
        self.regs.tim0().modify(|_, w| unsafe {
            w.se().bits(time.second);
            w.mi().bits(time.minute);
            w.ho().bits(time.hour);
            w.da().bits(time.day)
        });

        Self::wait_for_rtc_ctr_ready();
        self.regs.tim1().modify(|_, w| unsafe {
            w.dawe().bits(time.weekday as u8);
            w.mo().bits(time.month as u8);
            w.ye().bits(time.year)
        });
    }

    pub fn get_time(&self) -> Time {
        Time {
            second: self.regs.tim0().read().se().bits(),
            minute: self.regs.tim0().read().mi().bits(),
            hour: self.regs.tim0().read().ho().bits(),
            day: self.regs.tim0().read().da().bits(),
            weekday: Weekday::from(self.regs.tim1().read().dawe().bits()),
            month: Month::from(self.regs.tim1().read().mo().bits()),
            year: self.regs.tim1().read().ye().bits(),
        }
    }

    pub fn set_time_std_format(&self, time: Time) {
        let mut std_time = time;
        std_time.day = std_time
            .day
            .checked_sub(1)
            .expect("day must be at least 1 in std format");
        std_time.year = std_time
            .year
            .checked_add(YEAR_OFFSET)
            .expect("year overflow while converting to RTC format");
        self.set_time(std_time);
    }

    pub fn get_time_std_format(&self) -> Time {
        let mut time = self.get_time();
        time.day = time.day.saturating_add(1);
        time.year = time.year.saturating_sub(YEAR_OFFSET);
        time
    }

    pub fn set_alarm(&self, time: Alarm) {
        assert!(time.second <= MAX_SECONDS);
        assert!(time.minute <= MAX_MINUTES);
        assert!(time.hour <= MAX_HOURS);
        assert!(time.day < MAX_DAYS);
        assert!(time.year < MAX_YEAR);

        Self::wait_for_rtc_ctr_ready();
        self.regs.atim0().modify(|_, w| unsafe {
            w.ase().bits(time.second);
            w.ami().bits(time.minute);
            w.aho().bits(time.hour);
            w.ada().bits(time.day)
        });

        Self::wait_for_rtc_ctr_ready();
        self.regs.atim1().modify(|_, w| unsafe {
            w.amo().bits(time.month as u8);
            w.aye().bits(time.year)
        });
    }

    pub fn get_alarm(&self) -> Alarm {
        Alarm {
            second: self.regs.atim0().read().ase().bits(),
            minute: self.regs.atim0().read().ami().bits(),
            hour: self.regs.atim0().read().aho().bits(),
            day: self.regs.atim0().read().ada().bits(),
            month: Month::from(self.regs.atim1().read().amo().bits()),
            year: self.regs.atim1().read().aye().bits(),
        }
    }

    pub fn set_alarm_std_format(&self, time: Alarm) {
        let mut std_time = time;
        std_time.day = std_time
            .day
            .checked_sub(1)
            .expect("day must be at least 1 in std format");
        std_time.year = std_time
            .year
            .checked_add(YEAR_OFFSET)
            .expect("year overflow while converting to RTC format");
        self.set_alarm(std_time);
    }

    pub fn get_alarm_std_format(&self) -> Alarm {
        let mut time = self.get_alarm();
        time.day = time.day.saturating_add(1);
        time.year = time.year.saturating_sub(YEAR_OFFSET);
        time
    }

    pub fn get_event_status(&self) -> u32 {
        self.regs.stssr().read().bits()
    }

    pub fn enable(&self) {
        let scu = periph!(SCU_POWER);
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

    pub fn is_enabled(&self) -> bool {
        let power = unsafe { &*SCU_POWER::ptr() };
        let reset = unsafe { &*SCU_RESET::ptr() };
        power.pwrstat().read().hiben().bit_is_set() && !reset.rststat().read().hibrs().bit_is_set()
    }

    pub fn enable_event(&self, event: Event) {
        Self::wait_for_rtc_msksr_ready();
        match event {
            Event::Seconds => {
                self.regs.msksr().modify(|_, w| w.mpse().set_bit());
            }
            Event::Minutes => {
                self.regs.msksr().modify(|_, w| w.mpmi().set_bit());
            }
            Event::Hours => {
                self.regs.msksr().modify(|_, w| w.mpho().set_bit());
            }
            Event::Days => {
                self.regs.msksr().modify(|_, w| w.mpda().set_bit());
            }
            Event::Months => {
                self.regs.msksr().modify(|_, w| w.mpmo().set_bit());
            }
            Event::Years => {
                self.regs.msksr().modify(|_, w| w.mpye().set_bit());
            }
            Event::Alarm => {
                self.regs.msksr().modify(|_, w| w.mai().set_bit());
            }
        };
    }

    pub fn disable_event(&self, event: Event) {
        Self::wait_for_rtc_msksr_ready();
        match event {
            Event::Seconds => {
                self.regs.msksr().modify(|_, w| w.mpse().clear_bit());
            }
            Event::Minutes => {
                self.regs.msksr().modify(|_, w| w.mpmi().clear_bit());
            }
            Event::Hours => {
                self.regs.msksr().modify(|_, w| w.mpho().clear_bit());
            }
            Event::Days => {
                self.regs.msksr().modify(|_, w| w.mpda().clear_bit());
            }
            Event::Months => {
                self.regs.msksr().modify(|_, w| w.mpmo().clear_bit());
            }
            Event::Years => {
                self.regs.msksr().modify(|_, w| w.mpye().clear_bit());
            }
            Event::Alarm => {
                self.regs.msksr().modify(|_, w| w.mai().clear_bit());
            }
        };
    }

    pub fn clear_event(&self, event: Event) {
        Self::wait_for_rtc_clrsr_ready();
        self.regs
            .clrsr()
            .write(|w| unsafe { w.bits(u32::from(event)) });
    }

    pub fn enable_hibernation_wake_up(&self, event: WakeupEvent) {
        Self::wait_for_rtc_ctr_ready();
        match event {
            WakeupEvent::Alarm => {
                self.regs.ctr().modify(|_, w| w.tae().set_bit());
            }
            WakeupEvent::Seconds => {
                self.regs.ctr().modify(|_, w| w.esec().set_bit());
            }
            WakeupEvent::Minutes => {
                self.regs.ctr().modify(|_, w| w.emic().set_bit());
            }
            WakeupEvent::Hours => {
                self.regs.ctr().modify(|_, w| w.ehoc().set_bit());
            }
            WakeupEvent::Days => {
                self.regs.ctr().modify(|_, w| w.edac().set_bit());
            }
            WakeupEvent::Months => {
                self.regs.ctr().modify(|_, w| w.emoc().set_bit());
            }
            WakeupEvent::Years => {
                self.regs.ctr().modify(|_, w| w.eyec().set_bit());
            }
        };
    }

    pub fn disable_hibernation_wake_up(&self, event: WakeupEvent) {
        Self::wait_for_rtc_ctr_ready();
        match event {
            WakeupEvent::Alarm => {
                self.regs.ctr().modify(|_, w| w.tae().clear_bit());
            }
            WakeupEvent::Seconds => {
                self.regs.ctr().modify(|_, w| w.esec().clear_bit());
            }
            WakeupEvent::Minutes => {
                self.regs.ctr().modify(|_, w| w.emic().clear_bit());
            }
            WakeupEvent::Hours => {
                self.regs.ctr().modify(|_, w| w.ehoc().clear_bit());
            }
            WakeupEvent::Days => {
                self.regs.ctr().modify(|_, w| w.edac().clear_bit());
            }
            WakeupEvent::Months => {
                self.regs.ctr().modify(|_, w| w.emoc().clear_bit());
            }
            WakeupEvent::Years => {
                self.regs.ctr().modify(|_, w| w.eyec().clear_bit());
            }
        };
    }
}
