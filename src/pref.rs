pub trait PrefExt {
    fn constrain(self) -> Pref;
}

impl PrefExt for Pref {
    fn constrain(self) -> Pref {
        Pref {}
    }
}

pub struct Pref {}

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
