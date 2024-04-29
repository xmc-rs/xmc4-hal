pub trait CanExt {
    fn constrain(self) -> Can;
}

impl CanExt for Can {
    fn constrain(self) -> Can {
        Can {}
    }
}

pub struct Can {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE
