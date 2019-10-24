pub trait Ccu80Ext {
    fn constrain(self) -> Ccu80;
}

impl Ccu80Ext for Ccu80 {
    fn constrain(self) -> Ccu80 {
        Ccu80 {}
    }
}

pub struct Ccu80 {}

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
