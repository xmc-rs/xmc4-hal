pub trait DacExt {
    fn constrain(self) -> Dac;
}

impl DacExt for Dac {
    fn constrain(self) -> Dac {
        Dac {}
    }
}

pub struct Dac {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE
