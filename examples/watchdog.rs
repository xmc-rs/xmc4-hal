#![no_main]
#![no_std]

extern crate panic_semihosting;
use cortex_m_rt::entry;
use hal::pac;
use xmc4_hal as hal;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let watchdog = wdt::Wdt::new(p.WDT, scu::Scu::new());
    watchdog.start();
    loop {
        continue;
    }
}
