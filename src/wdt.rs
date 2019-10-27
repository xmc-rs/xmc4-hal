pub trait WdtExt {
    fn constrain(self) -> Wdt;
}

impl WdtExt for Wdt {
    fn constrain(self) -> Wdt {
        Wdt {}
    }
}

pub struct Wdt {}

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
