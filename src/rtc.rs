use crate::device::{RTC, SCU_GENERAL};

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
}
