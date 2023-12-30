use crate::pac::PREF;

pub trait PrefExt {
    fn constrain(self) -> Pref;
}

// impl PrefExt for Pref {
//     fn constrain(self) -> Pref {
//         Pref {}
//     }
// }

pub struct Pref {
    pub regs: PREF,
}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

impl Pref {
    pub fn new(pref: PREF) -> Self {
        Self { regs: pref }
    }

    pub fn enable_instruction_buffer(&self) {
        self.regs.pcon().write(|w| w.ibyp().clear_bit());
    }

    pub fn disable_instruction_buffer(&self) {
        self.regs.pcon().write(|w| w.ibyp().set_bit());
    }

    pub fn invalidate_instruction_buffer(&self) {
        self.regs.pcon().write(|w| w.iinv().set_bit());

        // Need calls to intrinsics __DSB and __ISB
        todo!();

        //self.regs.pcon().write(|w| w.iinv().clear_bit());

        // Need calls to intrinsics __DSB and __ISB
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
