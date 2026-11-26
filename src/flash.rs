use crate::pac::FLASH0;
use crate::sealed::Sealed;

pub trait FlashExt: Sealed {
    fn constrain(self) -> Flash;
}

impl Sealed for FLASH0 {}

impl FlashExt for FLASH0 {
    fn constrain(self) -> Flash {
        Flash::new(self)
    }
}

pub type PageData = [u32; 64];

const UNCACHED_BASE: u32 = 0xC000000;

const PROTECTION_CONFIRM_CODE: u32 = 0x8AFE15C3;

const PROTECTION_CONFIRM_OFFSET: u32 = 512;

const BYTES_PER_UCB: u32 = 1024;

const UCB0: u32 = UNCACHED_BASE;

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

/// Logical sectors in the XMC4 uncached map. Sectors 0-7 are 16 KiB, sector 8 is
/// 128 KiB, and every later sector is 256 KiB.
const SECTORS: [(u8, usize, usize); 16] = [
    (0, 0x0_0000, 0x4000),
    (1, 0x0_4000, 0x4000),
    (2, 0x0_8000, 0x4000),
    (3, 0x0_C000, 0x4000),
    (4, 0x1_0000, 0x4000),
    (5, 0x1_4000, 0x4000),
    (6, 0x1_8000, 0x4000),
    (7, 0x1_C000, 0x4000),
    (8, 0x2_0000, 0x2_0000),
    (9, 0x4_0000, 0x4_0000),
    (10, 0x8_0000, 0x4_0000),
    (11, 0xC_0000, 0x4_0000),
    (12, 0x10_0000, 0x4_0000),
    (13, 0x14_0000, 0x4_0000),
    (14, 0x18_0000, 0x4_0000),
    (15, 0x1C_0000, 0x4_0000),
];

pub struct FlashSectorIterator {
    index: usize,
    end: usize,
}

impl FlashSectorIterator {
    fn new(end: usize) -> Self {
        Self { index: 0, end }
    }
}

impl Iterator for FlashSectorIterator {
    type Item = FlashSector;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            return None;
        }
        let (number, offset, size) = SECTORS[self.index];
        self.index += 1;
        Some(FlashSector {
            number,
            offset,
            size,
        })
    }
}

/// Boot-mode index string programmed into user configuration block 2.
///
/// Layout matches `XMC_FLASH_BMI_STRING_t`, including the padding the C
/// compiler inserts before the checksum word.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BmiString {
    pub bmi: u32,
    pub mac_addr: [u8; 6],
    pub ip_extension: [u8; 16],
    pub usb_serial: [u8; 4],
    reserved: u32,
    reserved1: u16,
}

const _: () = assert!(core::mem::size_of::<BmiString>() == 40);

impl BmiString {
    pub fn new(bmi: u32) -> Self {
        Self {
            bmi,
            mac_addr: [0; 6],
            ip_extension: [0; 16],
            usb_serial: [0; 4],
            reserved: 0,
            reserved1: 0,
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

impl Flash {
    /// Logical sectors 0 through `count` (at most 16).
    pub fn sectors(&self, count: usize) -> FlashSectorIterator {
        FlashSectorIterator::new(count.min(SECTORS.len()))
    }
    pub fn new(flash: FLASH0) -> Self {
        Self { regs: flash }
    }

    pub fn is_busy(&self) -> bool {
        self.regs.fsr().read().pbusy().bit_is_set()
    }

    /// Enables the wait state for error correction process. It enables one additional wait state for ECC by setting WSECPF.
    pub fn enable_wait_state_for_ecc(&self) {
        self.regs.fcon().modify(|_, w| w.wsecpf().set_bit());
    }

    pub fn disable_wait_state_for_ecc(&self) {
        self.regs.fcon().modify(|_, w| w.wsecpf().clear_bit());
    }

    pub fn enable_dynamic_idle(&self) {
        self.regs.fcon().modify(|_, w| w.idle().set_bit());
    }

    pub fn disable_dynamic_idle(&self) {
        self.regs.fcon().modify(|_, w| w.idle().clear_bit());
    }

    pub fn enable_sleep_request(&self) {
        self.regs.fcon().modify(|_, w| w.sleep().set_bit());
    }

    pub fn disable_sleep_request(&self) {
        self.regs.fcon().modify(|_, w| w.sleep().clear_bit());
    }

    pub fn set_margin(&self, margin: Margin) {
        match margin {
            // TODO Should look at updating the SVD so that'value' methods have real names
            Margin::Default => self.regs.marp().modify(|_, w| w.margin().value1()),
            Margin::Tight0 => self.regs.marp().modify(|_, w| w.margin().value2()),
            Margin::Tight1 => self.regs.marp().modify(|_, w| w.margin().value3()),
        };
    }

    pub fn enable_double_bit_error_trap(&self) {
        self.regs.marp().modify(|_, w| w.trapdis().clear_bit());
    }

    pub fn disable_double_bit_error_trap(&self) {
        self.regs.marp().modify(|_, w| w.trapdis().set_bit());
    }

    pub fn set_wait_states(&self, wait_states: u8) {
        // TODO The SVD is weird on this and should be improved to not be a 4 value enum that really can do 16 numbers.
        self.regs
            .fcon()
            .modify(|_, w| unsafe { w.wspflash().bits(wait_states) });
    }

    /// Program a boot-mode index string into user configuration block 2.
    ///
    /// The checksum covers the first 30 bytes, matching `XMC_FLASH_InstallBMI`.
    pub fn install_bmi(&self, bmi: &BmiString) {
        let mut words = [0u32; 10];
        unsafe {
            let src = core::slice::from_raw_parts(bmi as *const BmiString as *const u8, 40);
            let mut checksum = 0u8;
            for byte in &src[..30] {
                checksum ^= *byte;
            }
            core::ptr::copy_nonoverlapping(src.as_ptr(), words.as_mut_ptr() as *mut u8, 32);
            words[8] = u32::from(checksum);
        }

        self.enter_page_mode_command();
        self.load_page_command(words[0], words[1]);
        self.load_page_command(words[2], words[3]);
        self.load_page_command(words[4], words[5]);
        self.load_page_command(words[6], words[7]);
        self.load_page_command(words[8], 0);

        let mut index = 0;
        while index < 64 - 10 {
            self.load_page_command(0, 0);
            index += 2;
        }

        let ucb2_page = (UNCACHED_BASE + 0x800 + 256) as *mut u32;
        self.write_ucb_page_command(ucb2_page);
        while self.regs.fsr().read().pbusy().bit_is_set() {}
    }

    pub fn install_protection(&self, user: User, mask: u32, password0: u32, password1: u32) {
        // TODO, explain where all numbers came from
        self.enter_page_mode_command();
        self.load_page_command(mask, 0);
        self.load_page_command(mask, 0);
        self.load_page_command(password0, password1);
        self.load_page_command(password0, password1);
        let mut index = 0;

        while index < 56 {
            self.load_page_command(0, 0);
            index += 2;
        }

        let start_address = (UCB0 + (user as u32 * BYTES_PER_UCB)) as *mut u32;

        self.write_ucb_page_command(start_address);

        while self.regs.fsr().read().pbusy().bit_is_set() {}
    }

    pub fn confirm_protection(&self, user: User) {
        // TODO, explain where all numbers came from
        self.enter_page_mode_command();
        self.load_page_command(PROTECTION_CONFIRM_CODE, 0);
        self.load_page_command(PROTECTION_CONFIRM_CODE, 0);
        let mut index = 0;

        while index < 60 {
            self.load_page_command(0, 0);
            index += 2;
        }

        let start_address = (UNCACHED_BASE + (user as u32 * 1024) + 512) as *mut u32;

        self.write_ucb_page_command(start_address);

        while self.regs.fsr().read().pbusy().bit_is_set() {}
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
            // User 2 configures read protection. It has no write-protection disable bit.
            let (disabled, procon) = match user {
                User::User0 => (
                    self.regs.fsr().read().wprodis0().bit_is_set(),
                    self.regs.procon0().read().bits(),
                ),
                User::User1 => (
                    self.regs.fsr().read().wprodis1().bit_is_set(),
                    self.regs.procon1().read().bits(),
                ),
                User::User2 => (false, 0),
            };

            result = disabled && procon == (_mask & !0x8000);
        }

        result
    }

    pub fn resume_protection(&self) {
        let address1 = (UNCACHED_BASE + 0x5554) as *mut u32;
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

    /// # Safety
    ///
    /// The caller must ensure `ucb_start_address` points to a valid UCB location in the
    /// flash memory region and that the device is in a state where the erase command sequence
    /// can execute safely.
    pub unsafe fn erase_ucb(&self, ucb_start_address: *mut u32) {
        let address1 = (UNCACHED_BASE + 0x5554) as *mut u32;
        let address2 = (UNCACHED_BASE + 0xAAA8) as *mut u32;

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
        let address1 = (UNCACHED_BASE + 0x5554) as *mut u32;
        unsafe {
            *address1 = 0xF0;
        }
    }

    fn enter_page_mode_command(&self) {
        let address = (UNCACHED_BASE + 0x5554) as *mut u32;

        unsafe {
            *address = 0x50;
        }
    }

    fn load_page_command(&self, low_word: u32, high_word: u32) {
        let address_low = (UNCACHED_BASE + 0x55F0) as *mut u32;
        let address_high = (UNCACHED_BASE + 0x55F4) as *mut u32;

        unsafe {
            *address_low = low_word;
            *address_high = high_word;
        }
    }

    fn write_page_command(&self, start_address: *mut u32) {
        let address1 = (UNCACHED_BASE + 0x5554) as *mut u32;
        let address2 = (UNCACHED_BASE + 0xAAA8) as *mut u32;

        unsafe {
            *address1 = 0xAA;
            *address2 = 0x55;
            *address1 = 0xA0;
            *start_address = 0xAA;
        }
    }

    fn write_ucb_page_command(&self, start_address: *mut u32) {
        let address1 = (UNCACHED_BASE + 0x5554) as *mut u32;
        let address2 = (UNCACHED_BASE + 0xAAA8) as *mut u32;

        unsafe {
            *address1 = 0xAA;
            *address2 = 0x55;
            *address1 = 0xC0;
            *start_address = 0xAA;
        }
    }

    fn erase_sector_command(&self, start_address: *mut u32) {
        let address1 = (UNCACHED_BASE + 0x5554) as *mut u32;
        let address2 = (UNCACHED_BASE + 0xAAA8) as *mut u32;

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
        let address1 = (UNCACHED_BASE + 0x5554) as *mut u32;
        let address2 = (UNCACHED_BASE + 0xAAA8) as *mut u32;
        let address3 = (UNCACHED_BASE + 0x553C) as *mut u32;
        let address4 = (UNCACHED_BASE + 0x5558) as *mut u32;

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
        let address1 = (UNCACHED_BASE + 0x5554) as *mut u32;
        let address2 = (UNCACHED_BASE + 0xAAA8) as *mut u32;
        let address3 = (UNCACHED_BASE + 0x553C) as *mut u32;
        let address4 = (UNCACHED_BASE + 0x5558) as *mut u32;

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
        let address1 = (UNCACHED_BASE + 0x5554) as *mut u32;
        let address2 = (UNCACHED_BASE + 0xAAA8) as *mut u32;

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
        let address1 = (UNCACHED_BASE + 0x5554) as *mut u32;
        let address2 = (UNCACHED_BASE + 0xAAA8) as *mut u32;
        let sector4 = (UNCACHED_BASE + 0x10000) as *mut u32;

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
        let address = (UNCACHED_BASE + 0x5554) as *mut u32;
        unsafe {
            *address = 0xF5;
        }
    }

    pub fn get_status(&self) -> u32 {
        self.regs.fsr().read().bits()
    }

    pub fn enable_event(&self, event: Event) {
        match event {
            Event::VerifyAndOperationError => self.regs.fcon().modify(|_, w| w.voperm().set_bit()),
            Event::CommandSequenceError => self.regs.fcon().modify(|_, w| w.sqerm().set_bit()),
            Event::ProtectionError => self.regs.fcon().modify(|_, w| w.proerm().set_bit()),
            Event::SingleBitError => self.regs.fcon().modify(|_, w| w.pfsberm().set_bit()),
            Event::DoubleBitError => self.regs.fcon().modify(|_, w| w.pfdberm().set_bit()),
            Event::EndOfBusy => self.regs.fcon().modify(|_, w| w.eobm().set_bit()),
        };
    }

    pub fn disable_event(&self, event: Event) {
        match event {
            Event::VerifyAndOperationError => {
                self.regs.fcon().modify(|_, w| w.voperm().clear_bit())
            }
            Event::CommandSequenceError => self.regs.fcon().modify(|_, w| w.sqerm().clear_bit()),
            Event::ProtectionError => self.regs.fcon().modify(|_, w| w.proerm().clear_bit()),
            Event::SingleBitError => self.regs.fcon().modify(|_, w| w.pfsberm().clear_bit()),
            Event::DoubleBitError => self.regs.fcon().modify(|_, w| w.pfdberm().clear_bit()),
            Event::EndOfBusy => self.regs.fcon().modify(|_, w| w.eobm().clear_bit()),
        };
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
