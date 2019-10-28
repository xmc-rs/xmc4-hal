pub trait PbaExt {
    fn constrain(self) -> Pba;
}

impl PbaExt for Pba {
    fn constrain(self) -> Pba {
        Pba {}
    }
}

pub struct Pba {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
