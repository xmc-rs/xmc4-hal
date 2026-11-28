//! USB device core, and the host core on XMC4300 and newer.
//!
//! This is the XMCLib device and host surface: connect, address, endpoint
//! read and write, stall, and events. It is not a USB class stack.

use core::ops::Deref;

use crate::pac::{
    self, USB0, USB0_EP0, USB0_EP1, USB0_EP2, USB0_EP3, USB0_EP4, USB0_EP5, USB0_EP6,
};
#[cfg(not(feature = "xmc4500"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;

#[cfg(not(any(feature = "xmc4100", feature = "xmc4200")))]
pub mod host;

const USB_BASE: usize = 0x5004_0000;

/// Transmit FIFO for one endpoint, and the shared receive FIFO at endpoint 0.
fn fifo(endpoint: u8) -> *mut u32 {
    (USB_BASE + 0x1000 + (endpoint as usize) * 0x1000) as *mut u32
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Event {
    StartOfFrame,
    Reset,
    EnumDone,
    InEndpoint,
    OutEndpoint,
}

pub trait UsbExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Usb;
}

impl Sealed for USB0 {}

impl UsbExt for USB0 {
    fn constrain(self, scu: &mut Scu) -> Usb {
        let usb = Usb { regs: self };
        usb.enable(scu);
        usb
    }
}

pub struct Usb {
    regs: USB0,
}

impl Usb {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(not(feature = "xmc4500"))]
        scu.ungate_peripheral_clock(PeripheralClock::Usb0);
        scu.deassert_peripheral_reset(PeripheralReset::Usb0);
        self.regs.grstctl().write(|w| w.csft_rst().set_bit());
    }

    pub fn disable(&self, scu: &mut Scu) {
        self.disconnect();
        scu.assert_peripheral_reset(PeripheralReset::Usb0);
        #[cfg(not(feature = "xmc4500"))]
        scu.gate_peripheral_clock(PeripheralClock::Usb0);
    }

    pub fn connect(&self) {
        self.regs.dctl().modify(|_, w| w.sft_discon().clear_bit());
    }

    pub fn disconnect(&self) {
        self.regs.dctl().modify(|_, w| w.sft_discon().set_bit());
    }

    pub fn set_address(&self, address: u8) -> Result<(), u8> {
        if address > 0x7f {
            return Err(address);
        }
        self.regs
            .dcfg()
            .modify(|_, w| unsafe { w.dev_addr().bits(address) });
        Ok(())
    }

    /// Device speed field, `0..=3`. Full speed on XMC is `1`.
    pub fn set_speed(&self, speed: u8) -> Result<(), u8> {
        if speed > 3 {
            return Err(speed);
        }
        self.regs
            .dcfg()
            .modify(|_, w| unsafe { w.dev_spd().bits(speed) });
        Ok(())
    }

    pub fn listen(&self, event: Event) {
        #[cfg(any(feature = "xmc4100", feature = "xmc4200"))]
        let mask = self.regs.gintmsk();
        #[cfg(not(any(feature = "xmc4100", feature = "xmc4200")))]
        let mask = self.regs.gintmsk_devicemode();
        mask.modify(|_, w| {
            match event {
                Event::StartOfFrame => w.sof_msk().set_bit(),
                Event::Reset => w.usbrst_msk().set_bit(),
                Event::EnumDone => w.enum_done_msk().set_bit(),
                Event::InEndpoint => w.iepint_msk().set_bit(),
                Event::OutEndpoint => w.oepint_msk().set_bit(),
            };
            w
        });
    }

    pub fn unlisten(&self, event: Event) {
        #[cfg(any(feature = "xmc4100", feature = "xmc4200"))]
        let mask = self.regs.gintmsk();
        #[cfg(not(any(feature = "xmc4100", feature = "xmc4200")))]
        let mask = self.regs.gintmsk_devicemode();
        mask.modify(|_, w| {
            match event {
                Event::StartOfFrame => w.sof_msk().clear_bit(),
                Event::Reset => w.usbrst_msk().clear_bit(),
                Event::EnumDone => w.enum_done_msk().clear_bit(),
                Event::InEndpoint => w.iepint_msk().clear_bit(),
                Event::OutEndpoint => w.oepint_msk().clear_bit(),
            };
            w
        });
    }

    pub fn is_event_pending(&self, event: Event) -> bool {
        #[cfg(any(feature = "xmc4100", feature = "xmc4200"))]
        let status = self.regs.gintsts().read();
        #[cfg(not(any(feature = "xmc4100", feature = "xmc4200")))]
        let status = self.regs.gintsts_devicemode().read();
        match event {
            Event::StartOfFrame => status.sof().bit_is_set(),
            Event::Reset => status.usbrst().bit_is_set(),
            Event::EnumDone => status.enum_done().bit_is_set(),
            Event::InEndpoint => status.iepint().bit_is_set(),
            Event::OutEndpoint => status.oepint().bit_is_set(),
        }
    }

    pub fn clear_event(&self, event: Event) {
        #[cfg(any(feature = "xmc4100", feature = "xmc4200"))]
        let status = self.regs.gintsts();
        #[cfg(not(any(feature = "xmc4100", feature = "xmc4200")))]
        let status = self.regs.gintsts_devicemode();
        status.write(|w| {
            match event {
                Event::StartOfFrame => {
                    w.sof().set_bit();
                }
                Event::Reset => {
                    w.usbrst().set_bit();
                }
                Event::EnumDone => {
                    w.enum_done().set_bit();
                }
                Event::InEndpoint | Event::OutEndpoint => {}
            }
            w
        });
    }

    /// Pop one receive-status word and, when it is an OUT data packet, copy the payload.
    pub fn read_out(&self, buffer: &mut [u8]) -> usize {
        #[cfg(any(feature = "xmc4100", feature = "xmc4200"))]
        let status = self.regs.grxstsp().read();
        #[cfg(not(any(feature = "xmc4100", feature = "xmc4200")))]
        let status = self.regs.grxstsp_devicemode().read();
        if status.pkt_sts().bits() != 2 {
            return 0;
        }
        let count = status.bcnt().bits() as usize;
        let take = count.min(buffer.len());
        unsafe {
            read_fifo(fifo(0), &mut buffer[..take]);
            if count > take {
                let mut discard = [0u8; 4];
                let mut left = count - take;
                while left > 0 {
                    let n = left.min(4);
                    read_fifo(fifo(0), &mut discard[..n]);
                    left -= n;
                }
            }
        }
        take
    }
}

pub trait Endpoint0Ext: Sealed {
    fn constrain(self) -> Endpoint0;
}

impl Sealed for USB0_EP0 {}

impl Endpoint0Ext for USB0_EP0 {
    fn constrain(self) -> Endpoint0 {
        Endpoint0 { regs: self }
    }
}

pub struct Endpoint0 {
    regs: USB0_EP0,
}

impl Endpoint0 {
    pub fn stall(&self) {
        self.regs.diepctl0().modify(|_, w| w.stall().set_bit());
        self.regs.doepctl0().modify(|_, w| w.stall().set_bit());
    }

    pub fn clear_stall(&self) {
        self.regs.diepctl0().modify(|_, w| w.stall().clear_bit());
        self.regs.doepctl0().modify(|_, w| w.stall().clear_bit());
    }

    pub fn write(&self, data: &[u8]) -> Result<(), usize> {
        if data.len() > 64 {
            return Err(data.len());
        }
        self.regs.dieptsiz0().write(|w| unsafe {
            w.xfer_size().bits(data.len() as u8);
            w.pkt_cnt().bits(1u8);
            w
        });
        self.regs.diepctl0().modify(|_, w| {
            w.epena().set_bit();
            w.cnak().set_bit();
            w
        });
        unsafe { write_fifo(fifo(0), data) };
        Ok(())
    }
}

pub trait EndpointExt: Sealed {
    fn constrain(self) -> Endpoint<Self>
    where
        Self: Sized;
}

macro_rules! endpoint_ext {
    ($($ty:ty, $number:expr),+) => {$(
        impl Sealed for $ty {}
        impl EndpointExt for $ty {
            fn constrain(self) -> Endpoint<Self> {
                Endpoint { regs: self, number: $number }
            }
        }
    )+};
}

endpoint_ext!(USB0_EP1, 1, USB0_EP2, 2, USB0_EP3, 3, USB0_EP4, 4, USB0_EP5, 5, USB0_EP6, 6);

pub struct Endpoint<E> {
    regs: E,
    number: u8,
}

impl<E> Endpoint<E>
where
    E: Deref<Target = pac::usb0_ep1::RegisterBlock>,
{
    /// Enable an interrupt or bulk endpoint. `kind` is the `EPTYPE` field, `0..=3`.
    pub fn enable(&self, kind: u8, max_packet: u16) -> Result<(), u8> {
        if kind > 3 || max_packet > 0x7ff {
            return Err(kind);
        }
        self.regs.diepctl_intbulk().modify(|_, w| unsafe {
            w.mps().bits(max_packet);
            w.eptype().bits(kind);
            w.tx_fnum().bits(self.number);
            w.usbact_ep().set_bit();
            w.set_d0pid().set_bit();
            w
        });
        self.regs.doepctl_intbulk().modify(|_, w| unsafe {
            w.eptype().bits(kind);
            w.usbact_ep().set_bit();
            w
        });
        Ok(())
    }

    pub fn stall(&self) {
        self.regs
            .diepctl_intbulk()
            .modify(|_, w| w.stall().set_bit());
        self.regs
            .doepctl_intbulk()
            .modify(|_, w| w.stall().set_bit());
    }

    pub fn clear_stall(&self) {
        self.regs
            .diepctl_intbulk()
            .modify(|_, w| w.stall().clear_bit());
        self.regs
            .doepctl_intbulk()
            .modify(|_, w| w.stall().clear_bit());
    }

    pub fn write(&self, data: &[u8], packet_size: u16) -> Result<(), usize> {
        if packet_size == 0 || data.len() > 0x7_ffff {
            return Err(data.len());
        }
        let packets = (data.len() / packet_size as usize).max(1) as u16;
        self.regs.dieptsiz().write(|w| unsafe {
            w.xfer_size().bits(data.len() as u32);
            w.pkt_cnt().bits(packets);
            w
        });
        self.regs.diepctl_intbulk().modify(|_, w| {
            w.epena().set_bit();
            w.cnak().set_bit();
            w
        });
        unsafe { write_fifo(fifo(self.number), data) };
        Ok(())
    }

    pub fn prepare_out(&self, bytes: u32, packets: u16) {
        self.regs.doeptsiz_control().write(|w| unsafe {
            w.xfer_size().bits(bytes);
            w.pkt_cnt().bits(packets);
            w
        });
        self.regs.doepctl_intbulk().modify(|_, w| {
            w.epena().set_bit();
            w.cnak().set_bit();
            w
        });
    }
}

unsafe fn write_fifo(fifo: *mut u32, data: &[u8]) {
    let mut offset = 0;
    while offset < data.len() {
        let mut word = 0u32;
        for byte in 0..4 {
            if offset + byte < data.len() {
                word |= u32::from(data[offset + byte]) << (8 * byte);
            }
        }
        core::ptr::write_volatile(fifo, word);
        offset += 4;
    }
}

unsafe fn read_fifo(fifo: *mut u32, data: &mut [u8]) {
    let mut offset = 0;
    while offset < data.len() {
        let word = core::ptr::read_volatile(fifo);
        for byte in 0..4 {
            if offset + byte < data.len() {
                data[offset + byte] = (word >> (8 * byte)) as u8;
            }
        }
        offset += 4;
    }
}
