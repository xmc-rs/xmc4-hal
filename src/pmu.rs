//! Power management unit identification.
//!
//! On XMC4 the PMU block exposed here is the module identification register.
//! Power, hibernate, and the EVR are controlled through the SCU.

use crate::pac::PMU0;
use crate::sealed::Sealed;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Id {
    pub revision: u8,
    pub module_type: u8,
    pub number: u16,
}

pub trait PmuExt: Sealed {
    fn constrain(self) -> Pmu;
}

impl Sealed for PMU0 {}

impl PmuExt for PMU0 {
    fn constrain(self) -> Pmu {
        Pmu { regs: self }
    }
}

pub struct Pmu {
    regs: PMU0,
}

impl Pmu {
    pub fn id(&self) -> Id {
        let id = self.regs.id().read();
        Id {
            revision: id.mod_rev().bits(),
            module_type: id.mod_type().bits(),
            number: id.mod_number().bits(),
        }
    }
}
