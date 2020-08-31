#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use hal::{device, rtc};
use xmc4_hal as hal;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let d = device::Peripherals::take().unwrap();

    loop {
        continue;
    }
}
