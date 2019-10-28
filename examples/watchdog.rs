#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use crate::hal::wdt::Wdt;
use xmc4_hal as hal;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let watchdog = Wdt::new();
    watchdog.start();
    loop {
        continue;
    }
}
