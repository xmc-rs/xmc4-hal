#![allow(dead_code)]

use crate::pac::{RTC, SCU_GENERAL, SCU_POWER, SCU_RESET};
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
#[derive(Copy, Clone, Debug)]
pub enum Event {
    Seconds = 0x01,
    Minutes = 0x02,
    Hours = 0x04,
    Days = 0x08,
    Months = 0x20,
    Years = 0x40,
    Alarm = 0x100,
}

impl From<u32> for Event {
    fn from(bits: u32) -> Self {
        match bits {
            0x01 => Event::Seconds,
            0x02 => Event::Minutes,
            0x04 => Event::Hours,
            0x08 => Event::Days,
            0x20 => Event::Months,
            0x40 => Event::Years,
            0x100 => Event::Alarm,
            _ => unimplemented!(),
        }
    }
}

impl From<Event> for u32 {
    fn from(bits: Event) -> Self {
        bits as u32
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
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

impl From<u8> for Month {
    fn from(val: u8) -> Self {
        match val {
            0 => Month::January,
            1 => Month::February,
            2 => Month::March,
            3 => Month::April,
            4 => Month::May,
            5 => Month::June,
            6 => Month::July,
            7 => Month::August,
            8 => Month::September,
            9 => Month::October,
            10 => Month::November,
            11 => Month::December,
            _ => unimplemented!(),
        }
    }
}

impl From<Month> for u8 {
    fn from(val: Month) -> Self {
        val as u8
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
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

impl From<u8> for Weekday {
    fn from(val: u8) -> Self {
        match val {
            0 => Weekday::Sunday,
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            6 => Weekday::Saturday,
            _ => Weekday::Sunday,
        }
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

// pub trait RtcExt {
//     fn constrain(self) -> Rtc;
// }

// impl RtcExt for RTC {
//     fn constrain(self) -> Rtc {
//         Rtc {}
//     }
// }

#[derive(Debug)]
pub struct Rtc {
    pub regs: RTC,
}

impl Rtc {
    pub fn new(rtc: RTC) -> Self {
        Self { regs: rtc }
    }

    pub fn start(&self) {
        let scu = unsafe { &*SCU_GENERAL::ptr() };
        while scu.mirrsts().read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        self.regs.ctr().write(|w| w.enb().set_bit());
    }

    pub fn stop(&self) {
        let scu = unsafe { &*SCU_GENERAL::ptr() };
        while scu.mirrsts().read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        self.regs.ctr().write(|w| w.enb().clear_bit());
    }

    pub fn is_running(&self) -> bool {
        let scu = unsafe { &*SCU_GENERAL::ptr() };
        while scu.mirrsts().read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        self.regs.ctr().read().enb().bit_is_set()
    }

    pub fn set_prescaler(&self, prescaler: u16) {
        let scu = unsafe { &*SCU_GENERAL::ptr() };
        while scu.mirrsts().read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        self.regs
            .ctr()
            .write(|w| unsafe { w.div().bits(prescaler) });
    }

    pub fn set_time(&self, time: Time) {
        assert!(time.second < MAX_SECONDS);
        assert!(time.minute < MAX_MINUTES);
        assert!(time.hour < MAX_HOURS);
        assert!(time.day < MAX_DAYS);
        assert!(time.year < MAX_YEAR);

        let scu = unsafe { &*SCU_GENERAL::ptr() };
        while scu.mirrsts().read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        self.regs.tim0().modify(|_, w| unsafe {
            w.se().bits(time.second);
            w.mi().bits(time.minute);
            w.ho().bits(time.hour);
            w.da().bits(time.day)
        });

        while scu.mirrsts().read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
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
        let mut std_time: Time = time;
        std_time.day -= 1;
        std_time.year += YEAR_OFFSET;
        self.set_time(std_time);
    }

    pub fn get_time_std_format(&self) -> Time {
        let mut time = self.get_time();
        time.day += 1;
        time.year -= YEAR_OFFSET;
        time
    }

    pub fn set_alarm(&self, time: Time) {
        assert!(time.second < MAX_SECONDS);
        assert!(time.minute < MAX_MINUTES);
        assert!(time.hour < MAX_HOURS);
        assert!(time.day < MAX_DAYS);
        assert!(time.year < MAX_YEAR);

        let scu = unsafe { &*SCU_GENERAL::ptr() };
        while scu.mirrsts().read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        self.regs.atim0().modify(|_, w| unsafe {
            w.ase().bits(time.second);
            w.ami().bits(time.minute);
            w.aho().bits(time.hour);
            w.ada().bits(time.day)
        });

        while scu.mirrsts().read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        self.regs.atim1().modify(|_, w| unsafe {
            w.amo().bits(time.month as u8);
            w.aye().bits(time.year)
        });
    }

    pub fn get_alarm(&self) -> Time {
        Time {
            second: self.regs.atim0().read().ase().bits(),
            minute: self.regs.atim0().read().ami().bits(),
            hour: self.regs.atim0().read().aho().bits(),
            day: self.regs.atim0().read().ada().bits(),
            weekday: Weekday::Sunday,
            month: Month::from(self.regs.atim1().read().amo().bits()),
            year: self.regs.atim1().read().aye().bits(),
        }
    }

    pub fn set_alarm_std_format(&self, time: Time) {
        let mut std_time: Time = time;
        std_time.day -= 1;
        std_time.year += YEAR_OFFSET;
        self.set_alarm(std_time);
    }

    pub fn get_alarm_std_format(&self) -> Time {
        let mut time = self.get_alarm();
        time.day += 1;
        time.year -= YEAR_OFFSET;
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
    }

    pub fn is_enabled(&self) -> bool {
        let power = unsafe { &*SCU_POWER::ptr() };
        let reset = unsafe { &*SCU_RESET::ptr() };
        return power.pwrstat().read().hiben().bit_is_set()
            && !reset.rststat().read().hibrs().bit_is_set();
    }

    fn enable_event(&self, event: Event) {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        while general.mirrsts().read().rtc_msksr().bit_is_set() {}
        match event {
            Event::Seconds => {
                self.regs.msksr().write(|w| w.mpse().set_bit());
            }
            Event::Minutes => {
                self.regs.msksr().write(|w| w.mpmi().set_bit());
            }
            Event::Hours => {
                self.regs.msksr().write(|w| w.mpho().set_bit());
            }
            Event::Days => {
                self.regs.msksr().write(|w| w.mpda().set_bit());
            }
            Event::Months => {
                self.regs.msksr().write(|w| w.mpmo().set_bit());
            }
            Event::Years => {
                self.regs.msksr().write(|w| w.mpye().set_bit());
            }
            Event::Alarm => {
                self.regs.msksr().write(|w| w.mai().set_bit());
            }
        };
    }

    fn disable_event(&self, event: Event) {
        let general = unsafe { &*SCU_GENERAL::ptr() };
        while general.mirrsts().read().rtc_msksr().bit_is_set() {}
        match event {
            Event::Seconds => {
                self.regs.msksr().write(|w| w.mpse().clear_bit());
            }
            Event::Minutes => {
                self.regs.msksr().write(|w| w.mpmi().clear_bit());
            }
            Event::Hours => {
                self.regs.msksr().write(|w| w.mpho().clear_bit());
            }
            Event::Days => {
                self.regs.msksr().write(|w| w.mpda().clear_bit());
            }
            Event::Months => {
                self.regs.msksr().write(|w| w.mpmo().clear_bit());
            }
            Event::Years => {
                self.regs.msksr().write(|w| w.mpye().clear_bit());
            }
            Event::Alarm => {
                self.regs.msksr().write(|w| w.mai().clear_bit());
            }
        };
    }

    fn clear_event(&self, event: Event) {
        let scu = unsafe { &(*SCU_GENERAL::ptr()) };
        while scu.mirrsts().read().rtc_clrsr().bit_is_set() {}
        self.regs
            .clrsr()
            .write(|w| unsafe { w.bits(u32::from(event)) });
    }

    fn enable_hibernation_wake_up(&self, event: WakeupEvent) {
        match event {
            WakeupEvent::Alarm => {
                self.regs.ctr().write(|w| w.tae().set_bit());
            }
            WakeupEvent::Seconds => {
                self.regs.ctr().write(|w| w.esec().set_bit());
            }
            WakeupEvent::Minutes => {
                self.regs.ctr().write(|w| w.emic().set_bit());
            }
            WakeupEvent::Hours => {
                self.regs.ctr().write(|w| w.ehoc().set_bit());
            }
            WakeupEvent::Days => {
                self.regs.ctr().write(|w| w.edac().set_bit());
            }
            WakeupEvent::Months => {
                self.regs.ctr().write(|w| w.emoc().set_bit());
            }
            WakeupEvent::Years => {
                self.regs.ctr().write(|w| w.eyec().set_bit());
            }
        };
    }

    fn disable_hibernation_wake_up(&self, event: WakeupEvent) {
        match event {
            WakeupEvent::Alarm => {
                self.regs.ctr().write(|w| w.tae().clear_bit());
            }
            WakeupEvent::Seconds => {
                self.regs.ctr().write(|w| w.esec().clear_bit());
            }
            WakeupEvent::Minutes => {
                self.regs.ctr().write(|w| w.emic().clear_bit());
            }
            WakeupEvent::Hours => {
                self.regs.ctr().write(|w| w.ehoc().clear_bit());
            }
            WakeupEvent::Days => {
                self.regs.ctr().write(|w| w.edac().clear_bit());
            }
            WakeupEvent::Months => {
                self.regs.ctr().write(|w| w.emoc().clear_bit());
            }
            WakeupEvent::Years => {
                self.regs.ctr().write(|w| w.eyec().clear_bit());
            }
        };
    }
}
