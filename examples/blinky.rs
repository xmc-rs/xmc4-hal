#![no_std]
#![no_main]

extern crate panic_semihosting;
use cortex_m_rt::entry;
use embedded_hal::digital::StatefulOutputPin;
use hal::pac;
use hal::port::PortExt;
use xmc4_hal as hal;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut led = p.PORT1.split().p1_0.into_push_pull_output();

    loop {
        led.toggle().ok();
    }
}
