/// Shameless reuse from https://github.com/stm32-rs

/// Bits per second
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Bps(pub u32);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Hertz(pub u32);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct KiloHertz(pub u32);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct MegaHertz(pub u32);

/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;

    /// Wrap in `Hertz`
    fn hz(self) -> Hertz;

    /// Wrap in `KiloHertz`
    fn khz(self) -> KiloHertz;

    /// Wrap in `MegaHertz`
    fn mhz(self) -> MegaHertz;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        Bps(self)
    }

    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn khz(self) -> KiloHertz {
        KiloHertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }
}

impl Into<Hertz> for KiloHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000)
    }
}

impl Into<KiloHertz> for Hertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 / 1_000)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000_000)
    }
}

impl Into<MegaHertz> for Hertz {
    fn into(self) -> MegaHertz {
        MegaHertz(self.0 / 1_000_000)
    }
}

impl Into<KiloHertz> for MegaHertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 * 1_000)
    }
}

impl Into<MegaHertz> for KiloHertz {
    fn into(self) -> MegaHertz {
        MegaHertz(self.0 / 1_000)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn to_hertz() {
        let hz32: u32 = 64;
        let hz = Hertz(64);
        assert_eq!(hz, hz32.hz());
    }

    #[test]
    fn to_kilohertz() {
        let hz32: u32 = 64;
        let hz = KiloHertz(64);
        assert_eq!(hz, hz32.khz());
    }

    #[test]
    fn to_megahertz() {
        let hz32: u32 = 64;
        let hz = MegaHertz(64);
        assert_eq!(hz, hz32.mhz());
    }

    #[test]
    fn to_bps() {
        let hz32: u32 = 64;
        let hz = Bps(64);
        assert_eq!(hz, hz32.bps());
    }

    #[test]
    fn to_hz_from_khz() {
        let hz = Hertz(55000);
        let khz = KiloHertz(55);
        assert_eq!(hz, khz.into());
    }

    #[test]
    fn to_hz_from_mhz() {
        let hz = Hertz(55000000);
        let mhz = MegaHertz(55);
        assert_eq!(hz, mhz.into());
    }

    #[test]
    fn to_khz_from_hz() {
        let hz = Hertz(55000000);
        let khz = KiloHertz(55000);
        assert_eq!(khz, hz.into());
    }

    #[test]
    fn to_khz_from_mhz() {
        let mhz = MegaHertz(55);
        let khz = KiloHertz(55000);
        assert_eq!(khz, mhz.into());
    }
}
