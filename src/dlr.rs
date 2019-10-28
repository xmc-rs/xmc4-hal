pub trait DlrExt {
    fn constrain(self) -> Dlr;
}

impl DlrExt for Dlr {
    fn constrain(self) -> Dlr {
        Dlr {}
    }
}

pub struct Dlr {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
