

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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
