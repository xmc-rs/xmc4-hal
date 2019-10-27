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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
