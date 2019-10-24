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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
