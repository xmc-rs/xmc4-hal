pub trait FceExt {
    fn constrain(self) -> Fce;
}

impl FceExt for Fce {
    fn constrain(self) -> Fce {
        Fce {}
    }
}

pub struct Fce {}

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
