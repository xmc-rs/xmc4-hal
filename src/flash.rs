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

pub struct Flash {
    pub regs: FLASH0,
}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

impl Flash {
    pub fn new(flash: FLASH0) -> Self {
        Self { regs: flash }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
