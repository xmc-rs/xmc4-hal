pub trait UsicExt {
    fn constrain(self) -> Usic;
}

impl UsicExt for Usic {
    fn constrain(self) -> Usic {
        Usic {}
    }
}

pub struct Usic {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE
