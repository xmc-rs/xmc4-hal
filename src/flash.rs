use crate::pac::FLASH0;

pub type PageData = [u32; 64];

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

#[derive(Copy, Clone, Debug)]
pub enum Margin {
    Default,
    Tight0,
    Tight1,
}

#[derive(Copy, Clone, Debug)]
pub enum User {
    User0,
    User1,
    User2,
}

impl From<User> for u32 {
    fn from(value: User) -> Self {
        value as u32
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    VerifyAndOperationError,
    CommandSequenceError,
    ProtectionError,
    SingleBitError,
    DoubleBitError,
    EndOfBusy,
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

    pub fn disable_sleep_request(&self) {
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

    pub fn verify_read_protection(&self, password0: u32, password1: u32) -> bool {
        let result = if self.regs.fsr().read().proin().bit_is_set() {
            self.clear_status();
            self.disable_read_protection_command(password0, password1);
            self.regs.fsr().read().rprodis().bit_is_set()
        } else {
            false
        };

        result
    }

    pub fn verify_write_protection(
        &self,
        user: User,
        _mask: u32,
        password0: u32,
        password1: u32,
    ) -> bool {
        let mut result = false;
        let wproinx = match user {
            User::User0 => self.regs.fsr().read().wproin0().bit_is_set(),
            User::User1 => self.regs.fsr().read().wproin1().bit_is_set(),
            User::User2 => self.regs.fsr().read().wproin2().bit_is_set(),
        };

        if wproinx {
            self.clear_status();
            self.disable_sector_write_protection_command(user, password0, password1);
            let wprodisx = match user {
                User::User0 => self.regs.fsr().read().wprodis0().bit_is_set(),
                User::User1 => self.regs.fsr().read().wprodis1().bit_is_set(),
                User::User2 => false, // Option does not exist, should not get here
            };

            result = wprodisx && self.regs.fsr().read().bits() == (_mask & 0xFFFF7FFF);
        }

        result
    }

    pub fn resume_protection(&self) {
        let address1 = 0xC005554 as *mut u32;
        unsafe {
            *address1 = 0x5E;
        }
    }

    pub fn repair_physical_sector(&self) {
        self.clear_status();
        self.repair_physical_sector_command();
    }

    pub fn erase_physical_sector(&self, address: *mut u32) {
        self.clear_status();
        self.erase_physical_sector_command(address);
        while self.regs.fsr().read().pbusy().bit_is_set() {}
    }

    pub fn erase_ucb(&self, ucb_start_address: *mut u32) {
        let address1 = 0xC005554 as *mut u32;
        let address2 = 0xC00aaa8 as *mut u32;

        unsafe {
            *address1 = 0xAA;
            *address2 = 0x55;
            *address1 = 0x80;
            *address1 = 0xAA;
            *address2 = 0x55;
            *ucb_start_address = 0xC0;
        }

        while self.regs.fsr().read().pbusy().bit_is_set() {}
    }

    /// Reset the status of the PFLASH
    pub fn reset(&self) {
        let address1 = 0xC005554 as *mut u32;
        unsafe {
            *address1 = 0xF0;
        }
    }

    fn enter_page_mode_command(&self) {
        let address = 0xC005554 as *mut u32;

        unsafe {
            *address = 0x50;
        }
    }

    fn load_page_command(&self, low_word: u32, high_word: u32) {
        let address_low = 0xC0055F0 as *mut u32;
        let address_high = 0xC0055F4 as *mut u32;

        unsafe {
            *address_low = low_word;
            *address_high = high_word;
        }
    }

    fn write_page_command(&self, start_address: *mut u32) {
        let address1 = 0xC005554 as *mut u32;
        let address2 = 0xC00AAA8 as *mut u32;

        unsafe {
            *address1 = 0xAA;
            *address2 = 0x55;
            *address1 = 0xA0;
            *start_address = 0xAA;
        }
    }

    fn write_ucb_page_command(&self, start_address: *mut u32) {
        let address1 = 0xC005554 as *mut u32;
        let address2 = 0xC00AAA8 as *mut u32;

        unsafe {
            *address1 = 0xAA;
            *address2 = 0x55;
            *address1 = 0xC0;
            *start_address = 0xAA;
        }
    }

    fn erase_sector_command(&self, start_address: *mut u32) {
        let address1 = 0xC005554 as *mut u32;
        let address2 = 0xC00aaa8 as *mut u32;

        unsafe {
            *address1 = 0xAA;
            *address2 = 0x55;
            *address1 = 0x80;
            *address1 = 0xAA;
            *address2 = 0x55;
            *start_address = 0x30;
        }
    }

    fn disable_sector_write_protection_command(&self, user: User, password0: u32, password1: u32) {
        let address1 = 0xC005554 as *mut u32;
        let address2 = 0xC00aaa8 as *mut u32;
        let address3 = 0xC00553C as *mut u32;
        let address4 = 0xC005558 as *mut u32;

        unsafe {
            *address1 = 0xAA;
            *address2 = 0x55;
            *address3 = user as u32;
            *address2 = password0;
            *address2 = password1;
            *address4 = 0x05;
        }
    }

    fn disable_read_protection_command(&self, password0: u32, password1: u32) {
        let address1 = 0xC005554 as *mut u32;
        let address2 = 0xC00aaa8 as *mut u32;
        let address3 = 0xC00553C as *mut u32;
        let address4 = 0xC005558 as *mut u32;

        unsafe {
            *address1 = 0x55;
            *address2 = 0xAA;
            *address3 = 0x00;
            *address2 = password0;
            *address2 = password1;
            *address4 = 0x08;
        }
    }

    fn erase_physical_sector_command(&self, start_address: *mut u32) {
        let address1 = 0xC005554 as *mut u32;
        let address2 = 0xC00aaa8 as *mut u32;

        unsafe {
            *address1 = 0xAA;
            *address2 = 0x55;
            *address1 = 0x80;
            *address1 = 0xAA;
            *address2 = 0x55;
            *start_address = 0x40;
        }
    }

    /// Command to erase physical sector4 which is starting with the specified address.
    /// This comand is only available if PROCON1.PRS = 1.
    fn repair_physical_sector_command(&self) {
        let address1 = 0xC005554 as *mut u32;
        let address2 = 0xC00aaa8 as *mut u32;
        let sector4 = 0xC010000 as *mut u32;

        unsafe {
            *address1 = 0xAA;
            *address2 = 0x55;
            *address1 = 0x80;
            *address1 = 0xAA;
            *address2 = 0x55;
            *sector4 = 0x40;
        }
    }

    pub fn clear_status(&self) {
        let address = 0xC005554 as *mut u32;
        unsafe {
            *address = 0xF5;
        }
    }

    pub fn get_status(&self) -> u32 {
        self.regs.fsr().read().bits()
    }

    pub fn enable_event(&self, event: Event) {
        match event {
            Event::VerifyAndOperationError => self.regs.fcon().write(|w| w.voperm().set_bit()),
            Event::CommandSequenceError => self.regs.fcon().write(|w| w.sqerm().set_bit()),
            Event::ProtectionError => self.regs.fcon().write(|w| w.proerm().set_bit()),
            Event::SingleBitError => self.regs.fcon().write(|w| w.pfsberm().set_bit()),
            Event::DoubleBitError => self.regs.fcon().write(|w| w.pfdberm().set_bit()),
            Event::EndOfBusy => self.regs.fcon().write(|w| w.eobm().set_bit()),
        }
    }

    pub fn disable_event(&self, event: Event) {
        match event {
            Event::VerifyAndOperationError => self.regs.fcon().write(|w| w.voperm().clear_bit()),
            Event::CommandSequenceError => self.regs.fcon().write(|w| w.sqerm().clear_bit()),
            Event::ProtectionError => self.regs.fcon().write(|w| w.proerm().clear_bit()),
            Event::SingleBitError => self.regs.fcon().write(|w| w.pfsberm().clear_bit()),
            Event::DoubleBitError => self.regs.fcon().write(|w| w.pfdberm().clear_bit()),
            Event::EndOfBusy => self.regs.fcon().write(|w| w.eobm().clear_bit()),
        }
    }

    pub fn program_page(&self, address: *mut u32, data: PageData) {
        let mut index = 0;

        self.clear_status();
        self.enter_page_mode_command();

        while index < data.len() {
            self.load_page_command(data[index], data[index + 1]);
            index += 2;
        }
        self.write_page_command(address);

        while self.regs.fsr().read().pbusy().bit_is_set() {}
    }

    pub fn erase_sector(&self, address: *mut u32) {
        self.clear_status();
        self.erase_sector_command(address);
        while self.regs.fsr().read().pbusy().bit_is_set() {}
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
