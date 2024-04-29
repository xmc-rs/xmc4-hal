pub trait GpdmaExt {
    fn constrain(self) -> Gpdma;
}

impl GpdmaExt for Gpdma {
    fn constrain(self) -> Gpdma {
        Gpdma {}
    }
}

pub struct Gpdma {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE
