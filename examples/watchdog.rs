#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use hal::{device, scu, wdt};
use xmc4_hal as hal;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let watchdog = wdt::Wdt::new(scu::Scu::new());
    watchdog.start();
    loop {
        continue;
    }
}
