//! Ethernet MAC.
//!
//! Address, descriptor lists, frame transmit and receive, link, and PHY
//! register access. There is no TCP/IP stack. The module is compiled only
//! when the selected device has `ETH0`.

use crate::pac::ETH0;
#[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;

const OWN: u32 = 1 << 31;
const TX_FIRST: u32 = 1 << 28;
const TX_LAST: u32 = 1 << 29;
const TX_INTERRUPT: u32 = 1 << 30;

/// Transmit descriptor. Place it in a region the DMA can read.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TxDescriptor {
    pub status: u32,
    pub size: u32,
    pub buffer: u32,
    pub next: u32,
}

impl TxDescriptor {
    pub const fn empty() -> Self {
        Self {
            status: 0,
            size: 0,
            buffer: 0,
            next: 0,
        }
    }

    /// Hand one buffer to the MAC. `len` is the frame length in bytes.
    pub fn prepare(&mut self, buffer: u32, len: u16) {
        self.size = u32::from(len) & 0x1fff;
        self.buffer = buffer;
        self.status = OWN | TX_INTERRUPT | TX_FIRST | TX_LAST;
    }

    pub fn done(&self) -> bool {
        self.status & OWN == 0
    }
}

/// Receive descriptor. Place it in a region the DMA can write.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RxDescriptor {
    pub status: u32,
    pub size: u32,
    pub buffer: u32,
    pub next: u32,
}

impl RxDescriptor {
    pub const fn empty() -> Self {
        Self {
            status: 0,
            size: 0,
            buffer: 0,
            next: 0,
        }
    }

    /// Give the DMA an empty buffer. `capacity` is the buffer size in bytes.
    pub fn arm(&mut self, buffer: u32, capacity: u16) {
        self.size = u32::from(capacity) & 0x1fff;
        self.buffer = buffer;
        self.status = OWN;
    }

    /// Frame length once the DMA has released the descriptor.
    pub fn received_len(&self) -> Option<u16> {
        if self.status & OWN != 0 {
            return None;
        }
        Some(((self.status >> 16) & 0x3fff) as u16)
    }
}

pub trait EthExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Eth;
}

impl Sealed for ETH0 {}

impl EthExt for ETH0 {
    fn constrain(self, scu: &mut Scu) -> Eth {
        let eth = Eth { regs: self };
        eth.enable(scu);
        eth
    }
}

pub struct Eth {
    regs: ETH0,
}

impl Eth {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
        scu.ungate_peripheral_clock(PeripheralClock::Eth0);
        scu.deassert_peripheral_reset(PeripheralReset::Eth0);
        self.regs.bus_mode().modify(|_, w| w.swr().set_bit());
    }

    pub fn disable(&self, scu: &mut Scu) {
        self.stop();
        scu.assert_peripheral_reset(PeripheralReset::Eth0);
        #[cfg(any(feature = "xmc4400", feature = "xmc4700", feature = "xmc4800"))]
        scu.gate_peripheral_clock(PeripheralClock::Eth0);
    }

    pub fn set_mac_address(&self, address: [u8; 6]) {
        let low = u32::from_le_bytes([address[0], address[1], address[2], address[3]]);
        let high = u16::from_le_bytes([address[4], address[5]]);
        self.regs
            .mac_address0_low()
            .write(|w| unsafe { w.addrlo().bits(low) });
        self.regs
            .mac_address0_high()
            .write(|w| unsafe { w.addrhi().bits(high) });
    }

    pub fn set_transmit_list(&self, address: u32) {
        self.regs
            .transmit_descriptor_list_address()
            .write(|w| unsafe { w.tdesla_32bit().bits(address >> 2) });
    }

    pub fn set_receive_list(&self, address: u32) {
        self.regs
            .receive_descriptor_list_address()
            .write(|w| unsafe { w.rdesla_32bit().bits(address >> 2) });
    }

    pub fn start(&self) {
        self.regs.mac_configuration().modify(|_, w| {
            w.te().set_bit();
            w.re().set_bit();
            w.dm().set_bit();
            w
        });
        self.regs.operation_mode().modify(|_, w| {
            w.st().set_bit();
            w.sr().set_bit();
            w
        });
    }

    pub fn stop(&self) {
        self.regs.operation_mode().modify(|_, w| {
            w.st().clear_bit();
            w.sr().clear_bit();
            w
        });
        self.regs.mac_configuration().modify(|_, w| {
            w.te().clear_bit();
            w.re().clear_bit();
            w
        });
    }

    /// PHY basic-status link bit.
    pub fn link_up(&self, phy: u8) -> Result<bool, ()> {
        let status = self.read_phy(phy, 1)?;
        Ok(status & (1 << 2) != 0)
    }

    pub fn read_phy(&self, phy: u8, register: u8) -> Result<u16, ()> {
        if phy > 31 || register > 31 {
            return Err(());
        }
        self.regs.gmii_address().modify(|_, w| unsafe {
            w.pa().bits(phy);
            w.mr().bits(register);
            w.mw().clear_bit();
            w.mb().set_bit();
            w
        });
        self.wait_mdio()?;
        Ok(self.regs.gmii_data().read().md().bits())
    }

    pub fn write_phy(&self, phy: u8, register: u8, data: u16) -> Result<(), ()> {
        if phy > 31 || register > 31 {
            return Err(());
        }
        self.regs
            .gmii_data()
            .write(|w| unsafe { w.md().bits(data) });
        self.regs.gmii_address().modify(|_, w| unsafe {
            w.pa().bits(phy);
            w.mr().bits(register);
            w.mw().set_bit();
            w.mb().set_bit();
            w
        });
        self.wait_mdio()
    }

    fn wait_mdio(&self) -> Result<(), ()> {
        for _ in 0..10_000 {
            if self.regs.gmii_address().read().mb().bit_is_clear() {
                return Ok(());
            }
        }
        Err(())
    }
}
