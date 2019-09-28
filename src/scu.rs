use crate::device::{SCU_POWER, SCU_RESET};

pub trait ScuExt {
    fn constrain(self) -> Scu;
}

pub struct Scu {}

impl ScuExt for Scu {
    fn constrain(self) -> Scu {
        Scu {}
    }
}

impl Scu {
    pub fn enable_hibernate_domain(&self) {
        let scu = unsafe { &*SCU_POWER::ptr() };
        if scu.pwrstat.read().hiben().bit_is_clear() {
            scu.pwrset.write(|w| w.hib().set_bit());
            while scu.pwrstat.read().hiben().bit_is_clear() {}
        }
    }
    pub fn is_hibernate_domain_enabled(&self) -> bool {
        let scu_power = unsafe { &*SCU_POWER::ptr() };
        let scu_reset = unsafe { &*SCU_RESET::ptr() };

        scu_power.pwrstat.read().hiben().bit_is_set()
            && !scu_reset.rststat.read().hibrs().bit_is_set()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
