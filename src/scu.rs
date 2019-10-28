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
        let scu = periph!(SCU_POWER);
        if scu.pwrstat.read().hiben().bit_is_clear() {
            scu.pwrset.write(|w| w.hib().set_bit());
            while scu.pwrstat.read().hiben().bit_is_clear() {}
        }
    }
    pub fn is_hibernate_domain_enabled(&self) -> bool {
        get_field!(SCU_POWER, pwrstat, hiben).bit_is_set()
            && !get_field!(SCU_RESET, rststat, hibrs).bit_is_set()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
