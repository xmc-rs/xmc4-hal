#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use crate::hal::{scu::Scu, wdt::Wdt};
use xmc4_hal as hal;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let scu = Scu::new();
    let watchdog = Wdt::new(scu);
    watchdog.start();
    loop {
        continue;
    }
}
