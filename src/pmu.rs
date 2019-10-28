pub trait PmuExt {
    fn constrain(self) -> Pmu;
}

impl PmuExt for Pmu {
    fn constrain(self) -> Pmu {
        Pmu {}
    }
}

pub struct Pmu {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
