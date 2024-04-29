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
