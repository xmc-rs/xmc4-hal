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

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
