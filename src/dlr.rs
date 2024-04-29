pub trait DlrExt {
    fn constrain(self) -> Dlr;
}

impl DlrExt for Dlr {
    fn constrain(self) -> Dlr {
        Dlr {}
    }
}

pub struct Dlr {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE
