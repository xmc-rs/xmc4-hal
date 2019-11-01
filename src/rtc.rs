#![allow(dead_code)]

use crate::device::{RTC, SCU_GENERAL, SCU_POWER, SCU_RESET};
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

pub enum Event {
    Seconds = 0x01,
    Minutes = 0x02,
    Hours = 0x04,
    Days = 0x08,
    Months = 0x20,
    Years = 0x40,
    Alarm = 0x100
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
        match bits {
            Event::Seconds => 0x01,
            Event::Minutes => 0x02,
            Event::Hours => 0x04,
            Event::Days => 0x08,
            Event::Months => 0x20,
            Event::Years => 0x40,
            Event::Alarm => 0x100
        }
    }
}

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
            _ => Month::January,
        }
    }
}

impl From<Month> for u8 {
    fn from(val: Month) -> Self {
        match val {
            Month::January => 0,
            Month::February => 1,
            Month::March => 2,
            Month::April => 3,
            Month::May => 4,
            Month::June => 5,
            Month::July => 6,
            Month::August => 7,
            Month::September => 8,
            Month::October => 9,
            Month::November => 10,
            Month::December => 11,
        }
    }
}

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
        match val {
            Weekday::Sunday => 0,
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
        }
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

pub enum WakeupEvent {
    Alarm,
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years
}

pub trait RtcExt {
    fn constrain(self) -> Rtc;
}

impl RtcExt for Rtc {
    fn constrain(self) -> Rtc {
        Rtc {}
    }
}

pub struct Rtc {}

impl Rtc {
    #[inline(always)]
    fn wait_for_mirrsts(&self) {
        while get_field!(SCU_GENERAL, mirrsts, rtc_ctr).bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
    }

    pub fn start(&self) {
        self.wait_for_mirrsts();
        set!(RTC, ctr, enb);
    }

    pub fn stop(&self) {
        self.wait_for_mirrsts();
        clear!(RTC, ctr, enb);
    }

    pub fn is_running(&self) -> bool {
        self.wait_for_mirrsts();
        get_field!(RTC, ctr, enb).bit_is_set()
    }

    pub fn set_prescaler(&self, prescaler: u16) {
        self.wait_for_mirrsts();
        set_field!(RTC, ctr, div, prescaler);
    }

    pub fn set_time(&self, time: Time) {
        assert!(time.second < MAX_SECONDS);
        assert!(time.minute < MAX_MINUTES);
        assert!(time.hour < MAX_HOURS);
        assert!(time.day < MAX_DAYS);
        assert!(time.year < MAX_YEAR);

        self.wait_for_mirrsts();
        let rtc = periph!(RTC);
        rtc.tim0.modify(|_, w| unsafe {
            w.se().bits(time.second);
            w.mi().bits(time.minute);
            w.ho().bits(time.hour);
            w.da().bits(time.day)
        });
        self.wait_for_mirrsts();
        rtc.tim1.modify(|_, w| unsafe {
            w.dawe().bits(time.weekday as u8);
            w.mo().bits(time.month as u8);
            w.ye().bits(time.year)
        });
    }

    pub fn get_time(&self) -> Time {
        let rtc = periph!(RTC);
        Time {
            second: rtc.tim0.read().se().bits(),
            minute: rtc.tim0.read().mi().bits(),
            hour: rtc.tim0.read().ho().bits(),
            day: rtc.tim0.read().da().bits(),
            weekday: Weekday::from(rtc.tim1.read().dawe().bits()),
            month: Month::from(rtc.tim1.read().mo().bits()),
            year: rtc.tim1.read().ye().bits(),
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

        self.wait_for_mirrsts();
        let rtc = periph!(RTC);
        rtc.atim0.modify(|_, w| unsafe {
            w.ase().bits(time.second);
            w.ami().bits(time.minute);
            w.aho().bits(time.hour);
            w.ada().bits(time.day)
        });
        self.wait_for_mirrsts();
        rtc.atim1.modify(|_, w| unsafe {
            w.amo().bits(time.month as u8);
            w.aye().bits(time.year)
        });
    }

    pub fn get_alarm(&self) -> Time {
        let rtc = periph!(RTC);
        Time {
            second: rtc.atim0.read().ase().bits(),
            minute: rtc.atim0.read().ami().bits(),
            hour: rtc.atim0.read().aho().bits(),
            day: rtc.atim0.read().ada().bits(),
            weekday: Weekday::Sunday,
            month: Month::from(rtc.atim1.read().amo().bits()),
            year: rtc.atim1.read().aye().bits(),
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
        get_reg!(RTC, stssr)
    }

    pub fn enable(&self) {
        let scu = periph!(SCU_POWER);
        if scu.pwrstat.read().hiben().bit_is_clear() {
            scu.pwrset.write(|w| w.hib().set_bit());
            while scu.pwrstat.read().hiben().bit_is_clear() {}
        }
    }

    pub fn is_enabled(&self) -> bool {
        get_field!(SCU_POWER, pwrstat, hiben).bit_is_set()
            && !get_field!(SCU_RESET, rststat, hibrs).bit_is_set()
    }

    fn enable_event(&self, event: Event) {
        while get_field!(SCU_GENERAL, mirrsts, rtc_msksr).bit_is_set() {}
        match event {
            Event::Seconds => {set!(RTC, msksr, mpse);},
            Event::Minutes => {set!(RTC, msksr, mpmi);},
            Event::Hours => {set!(RTC, msksr, mpho);},
            Event::Days => {set!(RTC, msksr, mpda);},
            Event::Months => {set!(RTC, msksr, mpmo);},
            Event::Years => {set!(RTC, msksr, mpye);},
            Event::Alarm => {set!(RTC, msksr, mai);}
        };
    }

    fn disable_event(&self, event: Event) {
        while get_field!(SCU_GENERAL, mirrsts, rtc_msksr).bit_is_set() {}
        match event {
            Event::Seconds => {clear!(RTC, msksr, mpse);},
            Event::Minutes => {clear!(RTC, msksr, mpmi);},
            Event::Hours => {clear!(RTC, msksr, mpho);},
            Event::Days => {clear!(RTC, msksr, mpda);},
            Event::Months => {clear!(RTC, msksr, mpmo);},
            Event::Years => {clear!(RTC, msksr, mpye);},
            Event::Alarm => {clear!(RTC, msksr, mai);}
        };
    }

    fn clear_event(&self, event: Event) {
        while get_field!(SCU_GENERAL, mirrsts, rtc_clrsr).bit_is_set() {}
        set_reg!(RTC, clrsr, u32::from(event));
    }

    fn enable_hibernation_wake_up(&self, event: WakeupEvent) {
        match event {
            WakeupEvent::Alarm => {set!(RTC, ctr, tae);},
            WakeupEvent::Seconds => {set!(RTC, ctr, esec);},
            WakeupEvent::Minutes => {set!(RTC, ctr, emic);},
            WakeupEvent::Hours => {set!(RTC, ctr, ehoc);},
            WakeupEvent::Days => {set!(RTC, ctr, edac);},
            WakeupEvent::Months => {set!(RTC, ctr, emoc);},
            WakeupEvent::Years => {set!(RTC, ctr, eyec);},
        };
    }

    fn disable_hibernation_wake_up(&self, event: WakeupEvent) {
        match event {
            WakeupEvent::Alarm => {clear!(RTC, ctr, tae);},
            WakeupEvent::Seconds => {clear!(RTC, ctr, esec);},
            WakeupEvent::Minutes => {clear!(RTC, ctr, emic);},
            WakeupEvent::Hours => {clear!(RTC, ctr, ehoc);},
            WakeupEvent::Days => {clear!(RTC, ctr, edac);},
            WakeupEvent::Months => {clear!(RTC, ctr, emoc);},
            WakeupEvent::Years => {clear!(RTC, ctr, eyec);},
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
