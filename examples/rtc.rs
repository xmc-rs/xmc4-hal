#![no_std]
#![no_main]

extern crate panic_semihosting;
use cortex_m_rt::entry;
use hal::pac;
use xmc4_hal as hal;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let rtc = hal::rtc::Rtc::new(p.RTC);

    rtc.enable();


    loop {
        continue;
    }
}
