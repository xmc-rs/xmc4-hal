#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use hal::rtc;
use xmc4_hal as hal;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let r = rtc::Rtc::new();
    r.start();
    loop {
        continue;
    }
}
