pub trait SdmmcExt {
    fn constrain(self) -> Sdmmc;
}

impl SdmmcExt for Sdmmc {
    fn constrain(self) -> Sdmmc {
        Sdmmc {}
    }
}

pub struct Sdmmc {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
