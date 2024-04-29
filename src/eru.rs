pub trait EruExt {
    fn constrain(self) -> Eru;
}

impl EruExt for Eru {
    fn constrain(self) -> Eru {
        Eru {}
    }
}

pub struct Eru {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE
