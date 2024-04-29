pub trait PosifExt {
    fn constrain(self) -> Posif;
}

impl PosifExt for Posif {
    fn constrain(self) -> Posif {
        Posif {}
    }
}

pub struct Posif {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE
