pub trait HrpwmExt {
    fn constrain(self) -> Hrpwm;
}

impl HrpwmExt for Hrpwm {
    fn constrain(self) -> Hrpwm {
        Hrpwm {}
    }
}

pub struct Hrpwm {}

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
