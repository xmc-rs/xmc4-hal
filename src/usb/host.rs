//! USB host pipes. Compiled only for XMC4300 and newer, which have host channels.

use crate::pac::{
    USB0_CH0, USB0_CH1, USB0_CH10, USB0_CH11, USB0_CH12, USB0_CH13, USB0_CH2, USB0_CH3, USB0_CH4,
    USB0_CH5, USB0_CH6, USB0_CH7, USB0_CH8, USB0_CH9,
};
use crate::sealed::Sealed;

use super::Usb;

impl Usb {
    pub fn port_power(&self, on: bool) {
        self.regs.hprt().modify(|_, w| {
            w.prt_conn_det().clear_bit();
            w.prt_en_chng().clear_bit();
            w.prt_ovr_curr_chng().clear_bit();
            if on {
                w.prt_pwr().set_bit();
            } else {
                w.prt_pwr().clear_bit();
            }
            w
        });
    }

    pub fn port_reset(&self, reset: bool) {
        self.regs.hprt().modify(|_, w| {
            w.prt_conn_det().clear_bit();
            w.prt_en_chng().clear_bit();
            w.prt_ovr_curr_chng().clear_bit();
            if reset {
                w.prt_rst().set_bit();
            } else {
                w.prt_rst().clear_bit();
            }
            w
        });
    }

    pub fn device_connected(&self) -> bool {
        self.regs.hprt().read().prt_conn_sts().bit_is_set()
    }

    pub fn port_enabled(&self) -> bool {
        self.regs.hprt().read().prt_ena().bit_is_set()
    }
}

pub trait PipeExt: Sealed {
    fn constrain(self) -> Pipe<Self>
    where
        Self: Sized;
}

macro_rules! pipe_ext {
    ($($ty:ty),+) => {$(
        impl Sealed for $ty {}
        impl PipeExt for $ty {
            fn constrain(self) -> Pipe<Self> {
                Pipe { regs: self }
            }
        }
    )+};
}

pipe_ext!(
    USB0_CH0, USB0_CH1, USB0_CH2, USB0_CH3, USB0_CH4, USB0_CH5, USB0_CH6, USB0_CH7, USB0_CH8,
    USB0_CH9, USB0_CH10, USB0_CH11, USB0_CH12, USB0_CH13
);

pub struct Pipe<C> {
    regs: C,
}

impl<C> Pipe<C>
where
    C: core::ops::Deref<Target = crate::pac::usb0_ch0::RegisterBlock>,
{
    /// Program the channel for one device endpoint. `kind` is `0..=3`.
    pub fn configure(
        &self,
        address: u8,
        endpoint: u8,
        kind: u8,
        max_packet: u16,
        inbound: bool,
    ) -> Result<(), u8> {
        if address > 0x7f || endpoint > 15 || kind > 3 {
            return Err(address);
        }
        self.regs.hcchar().modify(|_, w| unsafe {
            w.dev_addr().bits(address);
            w.epnum().bits(endpoint);
            w.eptype().bits(kind);
            w.mps().bits(max_packet);
            if inbound {
                w.epdir().set_bit();
            } else {
                w.epdir().clear_bit();
            }
            w
        });
        Ok(())
    }

    pub fn start(&self, bytes: u32, packets: u16) {
        self.regs.hctsiz_buffermode().write(|w| unsafe {
            w.xfer_size().bits(bytes);
            w.pkt_cnt().bits(packets);
            w
        });
        self.regs.hcchar().modify(|_, w| w.ch_ena().set_bit());
    }

    pub fn stop(&self) {
        self.regs.hcchar().modify(|_, w| w.ch_dis().set_bit());
    }
}
