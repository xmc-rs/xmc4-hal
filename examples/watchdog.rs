#![no_main]
#![no_std]

extern crate panic_semihosting;
use cortex_m_rt::entry;
use hal::pac;
use hal::scu::ScuExt;
use xmc4_hal as hal;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut scu = p.SCU_CLK.constrain();
    let watchdog = hal::wdt::Wdt::new(p.WDT, &mut scu);
    watchdog.start();
    loop {
        watchdog.service();
    }
}
