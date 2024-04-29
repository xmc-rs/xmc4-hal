pub trait LedtsExt {
    fn constrain(self) -> Ledts;
}

impl LedtsExt for Ledts {
    fn constrain(self) -> Ledts {
        Ledts {}
    }
}

pub struct Ledts {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE
