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

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
