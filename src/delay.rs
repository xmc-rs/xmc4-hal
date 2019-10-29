//! shameless reuse from https://github.com/stm32-rs/
//! API for delays with the systick timer
//!
//! Please be aware of potential overflows when using `delay_us`.
//! E.g. at 48MHz the maximum delay is 89 seconds.
//!
//! Consider using the timers api as a more flexible interface
//!
//! # Example
//!
//! ``` no_run
//! use xmc4_hal as hal;
//!
//! use crate::hal::delay::Delay;
//! use cortex_m::peripheral::Peripherals;
//!
//! let mut cp = cortex_m::Peripherals::take().unwrap();
//!
//! let mut delay = Delay::new(cp.SYST);
//! loop {
//!     delay.delay_ms(1_000_u16);
//! }
//! ```

use cast::{u16, u32};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use embedded_hal::blocking::delay::{DelayMs, DelayUs};

/// System timer (SysTick) as a delay provider
#[derive(Clone)]
pub struct Delay {
    scale: u32,
}

const SYSTICK_RANGE: u32 = 0x0100_0000;

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(mut syst: SYST) -> Delay {
        syst.set_clock_source(SystClkSource::Core);

        syst.set_reload(SYSTICK_RANGE - 1);
        syst.clear_current();
        syst.enable_counter();

        Delay { scale: 5 }
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, mut ms: u32) {
        const MAX_MS: u32 = 0x0000_FFFF;
        while ms != 0 {
            let current_ms = if ms <= MAX_MS { ms } else { MAX_MS };
            self.delay_us(current_ms * 1_000);
            ms -= current_ms;
        }
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        // Call delay_us directly, so we don't have to use the additional
        // delay loop the u32 variant uses
        self.delay_us(u32(ms) * 1_000);
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u16(ms));
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        const MAX_TICKS: u32 = 0x007F_FFFF;

        let mut total_ticks = us * self.scale;

        while total_ticks != 0 {
            let current_ticks = if total_ticks <= MAX_TICKS {
                total_ticks
            } else {
                MAX_TICKS
            };

            let start_count = SYST::get_current();
            total_ticks -= current_ticks;

            while (start_count.wrapping_sub(SYST::get_current()) % SYSTICK_RANGE) < current_ticks {}
        }
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(u32(us))
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(u32(us))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
