pub trait GpioExt {
    fn constrain(self) -> Gpio;
}

impl GpioExt for Gpio {
    fn constrain(self) -> Gpio {
        Gpio {}
    }
}

pub struct Gpio {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}