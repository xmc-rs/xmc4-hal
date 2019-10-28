pub trait FlashExt {
    fn constrain(self) -> Flash;
}

impl FlashExt for Flash {
    fn constrain(self) -> Flash {
        Flash {}
    }
}

pub struct Flash {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
