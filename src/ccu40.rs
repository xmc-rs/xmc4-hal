pub trait Ccu40Ext {
    fn constrain(self) -> Ccu40;
}

impl Ccu40Ext for Ccu40 {
    fn constrain(self) -> Ccu40 {
        Ccu40 {}
    }
}

pub struct Ccu40 {}

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
