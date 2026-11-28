//! EtherCAT slave controller.
//!
//! Init, port link, PHY register access, sync-manager enable, and the AL
//! event mask. Compiled for the devices that have ECAT0.

use core::ops::Deref;

use crate::pac::{
    self, ECAT0, ECAT0_SM0, ECAT0_SM1, ECAT0_SM2, ECAT0_SM3, ECAT0_SM4, ECAT0_SM5, ECAT0_SM6,
    ECAT0_SM7,
};
use crate::scu::{PeripheralClock, PeripheralReset, Scu};
use crate::sealed::Sealed;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AlEvent {
    AlControl,
    DcLatch,
    Sync0,
    Sync1,
    SyncManager,
}

pub trait EcatExt: Sealed {
    fn constrain(self, scu: &mut Scu) -> Ecat;
}

impl Sealed for ECAT0 {}

impl EcatExt for ECAT0 {
    fn constrain(self, scu: &mut Scu) -> Ecat {
        let ecat = Ecat { regs: self };
        ecat.enable(scu);
        ecat
    }
}

pub struct Ecat {
    regs: ECAT0,
}

impl Ecat {
    pub fn enable(&self, scu: &mut Scu) {
        #[cfg(not(feature = "xmc4500"))]
        scu.ungate_peripheral_clock(PeripheralClock::Ecat0);
        scu.deassert_peripheral_reset(PeripheralReset::Ecat0);
    }

    pub fn disable(&self, scu: &mut Scu) {
        scu.assert_peripheral_reset(PeripheralReset::Ecat0);
        #[cfg(not(feature = "xmc4500"))]
        scu.gate_peripheral_clock(PeripheralClock::Ecat0);
    }

    pub fn port_link(&self, port: u8) -> Result<bool, u8> {
        let status = self.regs.esc_dl_status().read();
        Ok(match port {
            0 => status.link_p0().bit_is_set(),
            1 => status.link_p1().bit_is_set(),
            2 => status.link_p2().bit_is_set(),
            3 => status.link_p3().bit_is_set(),
            _ => return Err(port),
        })
    }

    pub fn read_phy(&self, address: u8, register: u8) -> Result<u16, ()> {
        self.regs
            .mii_phy_adr()
            .write(|w| unsafe { w.phy_addr().bits(address) });
        self.regs
            .mii_phy_reg_adr()
            .write(|w| unsafe { w.phy_reg_addr().bits(register) });
        self.regs.mii_cont_stat().write(|w| w.cmd_reg().value2());
        self.wait_mii()?;
        if self.regs.mii_cont_stat().read().error().bit_is_set() {
            self.regs.mii_cont_stat().write(|w| w.cmd_reg().value1());
            return Err(());
        }
        Ok(self.regs.mii_phy_data().read().phy_rw_data().bits())
    }

    pub fn write_phy(&self, address: u8, register: u8, data: u16) -> Result<(), ()> {
        self.regs
            .mii_phy_adr()
            .write(|w| unsafe { w.phy_addr().bits(address) });
        self.regs
            .mii_phy_reg_adr()
            .write(|w| unsafe { w.phy_reg_addr().bits(register) });
        self.regs
            .mii_phy_data()
            .write(|w| unsafe { w.phy_rw_data().bits(data) });
        self.regs.mii_cont_stat().write(|w| w.cmd_reg().value3());
        self.wait_mii()?;
        if self.regs.mii_cont_stat().read().error().bit_is_set() {
            self.regs.mii_cont_stat().write(|w| w.cmd_reg().value1());
            return Err(());
        }
        Ok(())
    }

    fn wait_mii(&self) -> Result<(), ()> {
        for _ in 0..10_000 {
            if self.regs.mii_cont_stat().read().busy().bit_is_clear() {
                return Ok(());
            }
        }
        Err(())
    }

    pub fn enable_al_event(&self, event: AlEvent) {
        self.regs.al_event_mask().modify(|_, w| {
            match event {
                AlEvent::AlControl => w.al_ce_mask().set_bit(),
                AlEvent::DcLatch => w.dc_le_mask().set_bit(),
                AlEvent::Sync0 => w.st_s0_mask().set_bit(),
                AlEvent::Sync1 => w.st_s1_mask().set_bit(),
                AlEvent::SyncManager => w.sm_a_mask().set_bit(),
            };
            w
        });
    }

    pub fn disable_al_event(&self, event: AlEvent) {
        self.regs.al_event_mask().modify(|_, w| {
            match event {
                AlEvent::AlControl => w.al_ce_mask().clear_bit(),
                AlEvent::DcLatch => w.dc_le_mask().clear_bit(),
                AlEvent::Sync0 => w.st_s0_mask().clear_bit(),
                AlEvent::Sync1 => w.st_s1_mask().clear_bit(),
                AlEvent::SyncManager => w.sm_a_mask().clear_bit(),
            };
            w
        });
    }
}

pub trait SyncManagerExt: Sealed {
    fn constrain(self) -> SyncManager<Self>
    where
        Self: Sized;
}

macro_rules! sm_ext {
    ($($ty:ty),+) => {$(
        impl Sealed for $ty {}
        impl SyncManagerExt for $ty {
            fn constrain(self) -> SyncManager<Self> {
                SyncManager { regs: self }
            }
        }
    )+};
}

sm_ext!(ECAT0_SM0, ECAT0_SM1, ECAT0_SM2, ECAT0_SM3, ECAT0_SM4, ECAT0_SM5, ECAT0_SM6, ECAT0_SM7);

pub struct SyncManager<S> {
    regs: S,
}

impl<S> SyncManager<S>
where
    S: Deref<Target = pac::ecat0_sm0::RegisterBlock>,
{
    /// Clear the PDI deactivate bit so the sync manager controls its memory.
    pub fn enable(&self) {
        self.regs.sm_pdi_ctr().modify(|_, w| w.deact().clear_bit());
    }

    pub fn disable(&self) {
        self.regs.sm_pdi_ctr().modify(|_, w| w.deact().set_bit());
    }
}
