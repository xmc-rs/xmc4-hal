//! MultiCAN: node bit timing, message objects, transmit, receive, and acceptance filters.

use core::ops::Deref;

use crate::pac::{self, CAN, CAN_NODE0, CAN_NODE1};
#[cfg(not(feature = "xmc4500"))]
use crate::scu::PeripheralClock;
use crate::scu::{PeripheralReset, Scu};
use crate::sealed::Sealed;

#[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
use crate::pac::CAN_NODE2;
#[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
use crate::pac::{CAN_NODE3, CAN_NODE4, CAN_NODE5};

#[cfg(any(
    feature = "xmc4100",
    feature = "xmc4200",
    feature = "xmc4400",
    feature = "xmc4500"
))]
use crate::pac::CAN_MO0;

#[cfg(any(feature = "xmc4300", feature = "xmc4700", feature = "xmc4800"))]
use crate::pac::CAN_MO;

pub trait CanExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Can;
}

impl Sealed for CAN {}

impl CanExt for CAN {
    fn constrain(self, scu: &mut Scu) -> Can {
        let can = Can { regs: self };
        can.enable(scu);
        can
    }
}

pub struct Can {
    regs: CAN,
}

impl Can {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(not(feature = "xmc4500"))]
        scu.ungate_peripheral_clock(PeripheralClock::Mcan);
        scu.deassert_peripheral_reset(PeripheralReset::Mcan);
        self.regs.clc().modify(|_, w| w.disr().clear_bit());
    }

    pub fn disable(&self, scu: &mut Scu) {
        self.regs.clc().modify(|_, w| w.disr().set_bit());
        scu.assert_peripheral_reset(PeripheralReset::Mcan);
        #[cfg(not(feature = "xmc4500"))]
        scu.gate_peripheral_clock(PeripheralClock::Mcan);
    }
}

pub trait NodeExt: Sealed {
    fn constrain(self) -> Node<Self>
    where
        Self: Sized;
}

macro_rules! node_ext {
    ($($ty:ty),+) => {$(
        impl Sealed for $ty {}
        impl NodeExt for $ty {
            fn constrain(self) -> Node<Self> {
                Node { regs: self }
            }
        }
    )+};
}

node_ext!(CAN_NODE0, CAN_NODE1);
#[cfg(any(feature = "xmc4500", feature = "xmc4700", feature = "xmc4800"))]
node_ext!(CAN_NODE2);
#[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
node_ext!(CAN_NODE3, CAN_NODE4, CAN_NODE5);

pub struct Node<N> {
    regs: N,
}

impl<N> Node<N>
where
    N: Deref<Target = pac::can_node0::RegisterBlock>,
{
    /// Configuration change enable. Bit timing can be written in this state.
    pub fn enter_init(&self) {
        self.regs.ncr().modify(|_, w| {
            w.init().set_bit();
            w.cce().set_bit();
            w
        });
    }

    pub fn leave_init(&self) {
        self.regs.ncr().modify(|_, w| {
            w.cce().clear_bit();
            w.init().clear_bit();
            w
        });
    }

    /// Nominal bit timing. Values are the register fields (time segments are stored as count-1).
    pub fn set_bit_timing(&self, brp: u8, sjw: u8, tseg1: u8, tseg2: u8) -> Result<(), u8> {
        if brp > 63 || sjw > 3 || tseg1 > 15 || tseg2 > 7 {
            return Err(brp);
        }
        self.regs.nbtr().write(|w| unsafe {
            w.brp().bits(brp);
            w.sjw().bits(sjw);
            w.tseg1().bits(tseg1);
            w.tseg2().bits(tseg2);
            w
        });
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Frame {
    pub id: u16,
    pub data: [u8; 8],
    pub len: u8,
}

#[cfg(any(
    feature = "xmc4100",
    feature = "xmc4200",
    feature = "xmc4400",
    feature = "xmc4500"
))]
pub struct MessageObject<M> {
    regs: M,
}

#[cfg(any(
    feature = "xmc4100",
    feature = "xmc4200",
    feature = "xmc4400",
    feature = "xmc4500"
))]
impl<M> MessageObject<M>
where
    M: Deref<Target = pac::can_mo0::RegisterBlock>,
{
    pub fn new(regs: M) -> Self {
        Self { regs }
    }

    pub fn set_standard_id(&self, id: u16) -> Result<(), u16> {
        if id > 0x7ff {
            return Err(id);
        }
        self.regs.moar().modify(|_, w| unsafe {
            w.id().bits(u32::from(id) << 18);
            w.ide().clear_bit();
            w
        });
        Ok(())
    }

    pub fn set_filter(&self, id: u16, mask: u16) -> Result<(), u16> {
        if id > 0x7ff || mask > 0x7ff {
            return Err(id);
        }
        self.set_standard_id(id)?;
        self.regs.moamr().modify(|_, w| unsafe {
            w.am().bits(u32::from(mask) << 18);
            w.mide().set_bit();
            w
        });
        Ok(())
    }

    pub fn set_data(&self, data: &[u8]) -> Result<(), usize> {
        if data.len() > 8 {
            return Err(data.len());
        }
        let byte = |index: usize| data.get(index).copied().unwrap_or(0);
        self.regs
            .mofcr()
            .modify(|_, w| unsafe { w.dlc().bits(data.len() as u8) });
        self.regs.modatal().write(|w| unsafe {
            w.db0().bits(byte(0));
            w.db1().bits(byte(1));
            w.db2().bits(byte(2));
            w.db3().bits(byte(3));
            w
        });
        self.regs.modatah().write(|w| unsafe {
            w.db4().bits(byte(4));
            w.db5().bits(byte(5));
            w.db6().bits(byte(6));
            w.db7().bits(byte(7));
            w
        });
        Ok(())
    }

    pub fn transmit(&self) {
        self.regs.moctr().write(|w| {
            w.setmsgval().set_bit();
            w.settxen0().set_bit();
            w.settxen1().set_bit();
            w.setdir().set_bit();
            w.settxrq().set_bit();
            w
        });
    }

    pub fn enable_receive(&self) {
        self.regs.moctr().write(|w| {
            w.setmsgval().set_bit();
            w.setrxen().set_bit();
            w.resdir().set_bit();
            w
        });
    }

    pub fn receive(&self) -> Option<Frame> {
        let stat = self.regs.mostat().read();
        if stat.newdat().bit_is_clear() && stat.rxpnd().bit_is_clear() {
            return None;
        }
        let low = self.regs.modatal().read();
        let high = self.regs.modatah().read();
        let len = self.regs.mofcr().read().dlc().bits().min(8);
        self.regs.moctr().write(|w| {
            w.resnewdat().set_bit();
            w
        });
        Some(Frame {
            id: (self.regs.moar().read().id().bits() >> 18) as u16,
            data: [
                low.db0().bits(),
                low.db1().bits(),
                low.db2().bits(),
                low.db3().bits(),
                high.db4().bits(),
                high.db5().bits(),
                high.db6().bits(),
                high.db7().bits(),
            ],
            len,
        })
    }
}

#[cfg(any(
    feature = "xmc4100",
    feature = "xmc4200",
    feature = "xmc4400",
    feature = "xmc4500"
))]
impl Sealed for CAN_MO0 {}

#[cfg(any(feature = "xmc4300", feature = "xmc4700", feature = "xmc4800"))]
pub struct MessageObject {
    regs: CAN_MO,
    index: usize,
}

#[cfg(any(feature = "xmc4300", feature = "xmc4700", feature = "xmc4800"))]
impl MessageObject {
    pub fn new(regs: CAN_MO, index: usize) -> Result<Self, usize> {
        if index >= 64 {
            return Err(index);
        }
        Ok(Self { regs, index })
    }

    fn mo(&self) -> &pac::can_mo::mo::MO {
        self.regs.mo(self.index)
    }

    pub fn set_standard_id(&self, id: u16) -> Result<(), u16> {
        if id > 0x7ff {
            return Err(id);
        }
        self.mo().moar().modify(|_, w| unsafe {
            w.id().bits(u32::from(id) << 18);
            w.ide().clear_bit();
            w
        });
        Ok(())
    }

    pub fn set_filter(&self, id: u16, mask: u16) -> Result<(), u16> {
        if id > 0x7ff || mask > 0x7ff {
            return Err(id);
        }
        self.set_standard_id(id)?;
        self.mo().moamr().modify(|_, w| unsafe {
            w.am().bits(u32::from(mask) << 18);
            w.mide().set_bit();
            w
        });
        Ok(())
    }

    pub fn set_data(&self, data: &[u8]) -> Result<(), usize> {
        if data.len() > 8 {
            return Err(data.len());
        }
        let byte = |index: usize| data.get(index).copied().unwrap_or(0);
        self.mo()
            .mofcr()
            .modify(|_, w| unsafe { w.dlc().bits(data.len() as u8) });
        self.mo().modatal().write(|w| unsafe {
            w.db0().bits(byte(0));
            w.db1().bits(byte(1));
            w.db2().bits(byte(2));
            w.db3().bits(byte(3));
            w
        });
        self.mo().modatah().write(|w| unsafe {
            w.db4().bits(byte(4));
            w.db5().bits(byte(5));
            w.db6().bits(byte(6));
            w.db7().bits(byte(7));
            w
        });
        Ok(())
    }

    pub fn transmit(&self) {
        self.mo().moctr().write(|w| {
            w.setmsgval().set_bit();
            w.settxen0().set_bit();
            w.settxen1().set_bit();
            w.setdir().set_bit();
            w.settxrq().set_bit();
            w
        });
    }

    pub fn enable_receive(&self) {
        self.mo().moctr().write(|w| {
            w.setmsgval().set_bit();
            w.setrxen().set_bit();
            w.resdir().set_bit();
            w
        });
    }

    pub fn receive(&self) -> Option<Frame> {
        let stat = self.mo().mostat().read();
        if stat.newdat().bit_is_clear() && stat.rxpnd().bit_is_clear() {
            return None;
        }
        let low = self.mo().modatal().read();
        let high = self.mo().modatah().read();
        let len = self.mo().mofcr().read().dlc().bits().min(8);
        self.mo().moctr().write(|w| {
            w.resnewdat().set_bit();
            w
        });
        Some(Frame {
            id: (self.mo().moar().read().id().bits() >> 18) as u16,
            data: [
                low.db0().bits(),
                low.db1().bits(),
                low.db2().bits(),
                low.db3().bits(),
                high.db4().bits(),
                high.db5().bits(),
                high.db6().bits(),
                high.db7().bits(),
            ],
            len,
        })
    }
}
