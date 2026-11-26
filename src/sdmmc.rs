//! SD/MMC host: clock, commands, FIFO, bus width, and card detect.

use crate::pac::SDMMC;
#[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;

pub trait SdmmcExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Sdmmc;
}

impl Sealed for SDMMC {}

impl SdmmcExt for SDMMC {
    fn constrain(self, scu: &mut Scu) -> Sdmmc {
        let sdmmc = Sdmmc { regs: self };
        sdmmc.enable(scu);
        sdmmc
    }
}

pub struct Sdmmc {
    regs: SDMMC,
}

impl Sdmmc {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
        scu.ungate_peripheral_clock(PeripheralClock::Mmci);
        scu.deassert_peripheral_reset(PeripheralReset::Sdmmc);
        self.regs.sw_reset().write(|w| w.sw_rst_all().set_bit());
        self.regs
            .clock_ctrl()
            .modify(|_, w| w.internal_clock_en().set_bit());
    }

    pub fn disable(&self, scu: &mut Scu) {
        self.regs
            .clock_ctrl()
            .modify(|_, w| w.sdclock_en().clear_bit());
        scu.assert_peripheral_reset(PeripheralReset::Sdmmc);
        #[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
        scu.gate_peripheral_clock(PeripheralClock::Mmci);
    }

    pub fn power(&self, on: bool, voltage: u8) -> Result<(), u8> {
        if voltage > 7 {
            return Err(voltage);
        }
        self.regs.power_ctrl().modify(|_, w| unsafe {
            w.sd_bus_voltage_sel().bits(voltage);
            if on {
                w.sd_bus_power().set_bit();
            } else {
                w.sd_bus_power().clear_bit();
            }
            w
        });
        Ok(())
    }

    pub fn start_clock(&self, divider: u8) {
        self.regs.clock_ctrl().modify(|_, w| unsafe {
            w.sdclk_freq_sel().bits(divider);
            w.internal_clock_en().set_bit();
            w.sdclock_en().set_bit();
            w
        });
    }

    /// Bus width of 1, 4, or 8 bits.
    pub fn set_bus_width(&self, bits: u8) -> Result<(), u8> {
        if bits != 1 && bits != 4 && bits != 8 {
            return Err(bits);
        }
        self.regs.host_ctrl().modify(|_, w| {
            match bits {
                4 => {
                    w.data_tx_width().set_bit();
                    w.sd_8bit_mode().clear_bit();
                }
                8 => {
                    w.data_tx_width().clear_bit();
                    w.sd_8bit_mode().set_bit();
                }
                _ => {
                    w.data_tx_width().clear_bit();
                    w.sd_8bit_mode().clear_bit();
                }
            }
            w
        });
        Ok(())
    }

    pub fn set_block(&self, size: u16, count: u16) {
        self.regs
            .block_size()
            .modify(|_, w| unsafe { w.tx_block_size().bits(size) });
        self.regs
            .block_count()
            .write(|w| unsafe { w.block_count().bits(count) });
    }

    pub fn command(&self, index: u8, argument: u32, response: u8, data: bool) -> Result<(), u8> {
        if index > 63 || response > 3 {
            return Err(index);
        }
        self.regs
            .argument1()
            .write(|w| unsafe { w.argument1().bits(argument) });
        self.regs.command().write(|w| unsafe {
            w.cmd_ind().bits(index);
            w.resp_type_select().bits(response);
            w.cmd_crc_check_en().set_bit();
            w.cmd_ind_check_en().set_bit();
            if data {
                w.data_present_select().set_bit();
            }
            w
        });
        Ok(())
    }

    pub fn response(&self) -> u32 {
        u32::from(self.regs.response0().read().response0().bits())
    }

    pub fn card_inserted(&self) -> bool {
        self.regs
            .present_state()
            .read()
            .card_inserted()
            .bit_is_set()
    }

    pub fn write_protected(&self) -> bool {
        self.regs
            .present_state()
            .read()
            .write_protect_pin_level()
            .bit_is_clear()
    }

    pub fn read_fifo(&self) -> u32 {
        self.regs.data_buffer().read().data_buffer().bits()
    }

    pub fn write_fifo(&self, word: u32) {
        self.regs
            .data_buffer()
            .write(|w| unsafe { w.data_buffer().bits(word) });
    }
}
