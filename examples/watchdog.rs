#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use xmc4_hal as hal;
use hal::{device, scu, wdt};


use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let watchdog = wdt::Wdt::new(scu::Scu::new());
    watchdog.start();
    loop {
        continue;
    }
}
