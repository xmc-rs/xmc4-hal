

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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
