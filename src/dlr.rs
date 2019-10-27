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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
