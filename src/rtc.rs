use crate::device::{RTC, SCU_GENERAL};

const MAX_SECONDS: u8 = 59;
const MAX_MINUTES: u8 = 59;
const MAX_HOURS: u8 = 23;
const MAX_DAYS: u8 = 31;
const MAX_YEAR: u16 = 65535;
const YEAR_OFFSET: u16 = 1900;

pub enum Event {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

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

pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

pub struct Time {
    second: u8,
    minute: u8,
    hour: u8,
    day: u8,
    weekday: Weekday,
    month: Month,
    year: u16,
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
    pub fn start(self) {
        let rtc = unsafe { &*RTC::ptr() };
        let scu_gen = unsafe { &*SCU_GENERAL::ptr() };

        while scu_gen.mirrsts.read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        rtc.ctr.modify(|_, w| w.enb().set_bit());
    }

    pub fn stop(self) {
        let rtc = unsafe { &*RTC::ptr() };
        let scu_gen = unsafe { &*SCU_GENERAL::ptr() };

        while scu_gen.mirrsts.read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        rtc.ctr.modify(|_, w| w.enb().clear_bit());
    }

    pub fn is_running(self) -> bool {
        let rtc = unsafe { &*RTC::ptr() };
        let scu_gen = unsafe { &*SCU_GENERAL::ptr() };

        while scu_gen.mirrsts.read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        rtc.ctr.read().enb().bit_is_set()
    }

    pub fn set_prescaler(prescaler: u16) {
        let scu_gen = unsafe { &*SCU_GENERAL::ptr() };
        while scu_gen.mirrsts.read().rtc_ctr().bit_is_clear() {
            // Check SCU_MIRRSTS to ensure that no transfer over serial interface is pending
        }
        let rtc = unsafe { &*RTC::ptr() };
        rtc.ctr.modify(|_, w| unsafe { w.div().bits(prescaler) });
    }

    pub fn set_time(time: Time) {
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
}
