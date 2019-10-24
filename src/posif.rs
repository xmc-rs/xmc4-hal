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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
