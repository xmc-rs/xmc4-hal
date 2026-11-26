#![no_std]
#![no_main]

extern crate panic_semihosting;
use cortex_m_rt::entry;
use embedded_hal_nb::serial::{Read, Write};
use hal::pac;
use hal::port::PortExt;
use hal::scu::ScuExt;
use hal::serial::Serial;
use hal::time::U32Ext;
use hal::usic::UsicExt;
use xmc4_hal as hal;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut scu = p.SCU_CLK.constrain();
    let clocks = scu.cfgr().freeze();
    let usic = p.USIC0.constrain(&mut scu);
    let channel = usic.channel(p.USIC0_CH0);

    let port = p.PORT1.split();
    let tx = port.p1_0.into_alternate_push_pull::<2>();
    let rx = port.p1_1.into_alternate_push_pull::<2>();
    let mut serial = Serial::new(channel, tx, rx, &clocks, 115_200.bps()).unwrap();

    loop {
        if let Ok(byte) = serial.read() {
            let _ = serial.write(byte);
        }
    }
}
