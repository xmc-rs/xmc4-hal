use crate::device::{RTC, SCU_GENERAL};
use crate::scu::Scu;

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
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
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

impl Month {
    fn from(val: u8) -> Month {
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
    fn from(val: u8) -> Weekday {
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
    pub fn start(&self) {
        let rtc = unsafe { &*RTC::ptr() };
        let scu_gen = unsafe { &*SCU_GENERAL::ptr() };

        while scu_gen.mirrsts.read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        rtc.ctr.modify(|_, w| w.enb().set_bit());
    }

    pub fn stop(&self) {
        let rtc = unsafe { &*RTC::ptr() };
        let scu_gen = unsafe { &*SCU_GENERAL::ptr() };

        while scu_gen.mirrsts.read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        rtc.ctr.modify(|_, w| w.enb().clear_bit());
    }

    pub fn is_running(&self) -> bool {
        let rtc = unsafe { &*RTC::ptr() };
        let scu_gen = unsafe { &*SCU_GENERAL::ptr() };

        while scu_gen.mirrsts.read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        rtc.ctr.read().enb().bit_is_set()
    }

    pub fn set_prescaler(&self, prescaler: u16) {
        let scu_gen = unsafe { &*SCU_GENERAL::ptr() };
        while scu_gen.mirrsts.read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        let rtc = unsafe { &*RTC::ptr() };
        rtc.ctr.modify(|_, w| unsafe { w.div().bits(prescaler) });
    }

    pub fn set_time(&self, time: Time) {
        assert!(time.second < MAX_SECONDS);
        assert!(time.minute < MAX_MINUTES);
        assert!(time.hour < MAX_HOURS);
        assert!(time.day < MAX_DAYS);
        assert!(time.year < MAX_YEAR);

        let scu_gen = unsafe { &*SCU_GENERAL::ptr() };
        while scu_gen.mirrsts.read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        let rtc = unsafe { &*RTC::ptr() };
        // TODO: Not sure if this is fully correct. The C code does a single struct assignment.
        rtc.tim0.modify(|_, w| unsafe {
            w.se().bits(time.second);
            w.mi().bits(time.minute);
            w.ho().bits(time.hour);
            w.da().bits(time.day)
        });
        while scu_gen.mirrsts.read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        rtc.tim1.modify(|_, w| unsafe {
            w.dawe().bits(time.weekday as u8);
            w.mo().bits(time.month as u8);
            w.ye().bits(time.year)
        });
    }

    pub fn get_time(&self) -> Time {
        let rtc = unsafe { &*RTC::ptr() };
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

        let scu_gen = unsafe { &*SCU_GENERAL::ptr() };
        while scu_gen.mirrsts.read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        let rtc = unsafe { &*RTC::ptr() };
        // TODO: Not sure if this is fully correct. The C code does a single struct assignment.
        rtc.atim0.modify(|_, w| unsafe {
            w.ase().bits(time.second);
            w.ami().bits(time.minute);
            w.aho().bits(time.hour);
            w.ada().bits(time.day)
        });
        while scu_gen.mirrsts.read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        rtc.atim1.modify(|_, w| unsafe {
            w.amo().bits(time.month as u8);
            w.aye().bits(time.year)
        });
    }

    pub fn get_alarm(&self) -> Time {
        let rtc = unsafe { &*RTC::ptr() };
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
        let rtc = unsafe { &*RTC::ptr() };
        rtc.stssr.read().bits()
    }

    pub fn enable(&self, scu_reg: &Scu){
        /// TODO Address correct usage of register access
        scu_reg.enable_hibernate_domain();
    }

    pub fn is_enabled(&self, scu_reg: &Scu) -> bool {
        /// TODO Address correct usage of register access
        scu_reg.is_hibernate_domain_enabled()
    }
}
