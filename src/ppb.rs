pub trait PpbExt {
    fn constrain(self) -> Ppb;
}

impl PpbExt for Ppb {
    fn constrain(self) -> Ppb {
        Ppb {}
    }
}

pub struct Ppb {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
