

pub trait VadcExt {
    fn constrain(self) -> Vadc;
}

impl VadcExt for Vadc {
    fn constrain(self) -> Vadc {
        Vadc {}
    }
}

pub struct Vadc {}

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
