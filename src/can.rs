
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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
