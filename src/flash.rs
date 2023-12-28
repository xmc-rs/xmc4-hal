use crate::pac::FLASH0;

pub struct FlashSector {
    pub number: u8,
    pub offset: usize,
    pub size: usize,
}

impl FlashSector {
    pub fn contains(&self, offset: usize) -> bool {
        self.offset <= offset && offset < self.offset + self.size
    }
}

pub struct FlashSectorIterator {
    index: u8,
    start_sector: u8,
    start_offset: usize,
    end_offset: usize,
}

impl FlashSectorIterator {
    fn new(start_sector: u8, start_offset: usize, end_offset: usize) -> Self {
        Self {
            index: 0,
            start_sector,
            start_offset,
            end_offset,
        }
    }
}

impl Iterator for FlashSectorIterator {
    type Item = FlashSector;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start_offset >= self.end_offset {
            None
        } else {
            let size = match self.index {
                8..=11 => 0x20000,
                _ => 0x4000,
            };

            let sector = FlashSector {
                number: self.start_sector + self.index,
                offset: self.start_offset,
                size,
            };
            self.index += 1;
            self.start_offset += size;

            Some(sector)
        }
    }
}

pub enum Margin {
    Default,
    Tight0,
    Tight1,
}

pub struct Flash {
    pub regs: FLASH0,
}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

impl Flash {
    pub fn new(flash: FLASH0) -> Self {
        Self { regs: flash }
    }

    /// Enables the wait state for error correction process. It enables one additional wait state for ECC by setting WSECPF.
    pub fn enable_wait_state_for_ecc(&self) {
        self.regs.fcon().write(|w| w.wsecpf().set_bit());
    }

    pub fn disable_wait_state_for_ecc(&self) {
        self.regs.fcon().write(|w| w.wsecpf().clear_bit());
    }

    pub fn enable_dynamic_idle(&self) {
        self.regs.fcon().write(|w| w.idle().set_bit());
    }

    pub fn disable_dynamic_idle(&self) {
        self.regs.fcon().write(|w| w.idle().clear_bit());
    }

    pub fn enable_sleep_request(&self) {
        self.regs.fcon().write(|w| w.sleep().set_bit());
    }

    pub fn disable_slee_request(&self) {
        self.regs.fcon().write(|w| w.sleep().clear_bit());
    }

    pub fn set_margin(&self, margin: Margin) {
        match margin {
            // TODO Should look at updating the SVD so that'value' methods have real names
            Margin::Default => self.regs.marp().write(|w| w.margin().value1()),
            Margin::Tight0 => self.regs.marp().write(|w| w.margin().value2()),
            Margin::Tight1 => self.regs.marp().write(|w| w.margin().value3()),
        }
    }

    pub fn enable_double_bit_error_trap(&self) {
        self.regs.marp().write(|w| w.trapdis().clear_bit());
    }

    pub fn disable_double_bit_error_trap(&self) {
        self.regs.marp().write(|w| w.trapdis().set_bit());
    }

    pub fn set_wait_states(&self, wait_states: u8) {
        // TODO The SVD is weird on this and should be improved to not be a 4 value enum that really can do 16 numbers.
        self.regs
            .fcon()
            .write(|w| unsafe { w.wspflash().bits(wait_states) });
    }

    pub fn enable_instruction_buffer(&self) {
        // TODO Not sure if i want this here
        unimplemented!();
    }

    pub fn disable_instruction_buffer(&self) {
        // TODO Not sure if i want this here
        unimplemented!();
    }

    pub fn invalidate_instruction_buffer(&self) {
        // TODO Not sure if i need or want this, or even have it here
        unimplemented!();
    }

    pub fn install_bmi(&self) {
        unimplemented!();
    }

    pub fn install_protection(&self, _user: u8, _mask: u32, _password0: u32, _password1: u32) {
        unimplemented!();
    }

    pub fn confirm_protection(&self, _user: u8) {
        unimplemented!();
    }

    pub fn verify_read_protection(&self, _password0: u32, _password1: u32) {
        unimplemented!();
    }

    pub fn verify_write_protection(&self, _user: u8, _mask: u32, _password0: u32, _password1: u32) {
        unimplemented!();
    }

    pub fn resume_protection(&self) {
        unimplemented!();
    }

    pub fn repair_physical_sector(&self) {
        unimplemented!();
    }

    pub fn erase_physical_sector(&self, _address: &u32) {
        unimplemented!();
    }

    pub fn erase_ucb(&self, _address: &u32) {
        unimplemented!();
    }

    pub fn reset(&self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
