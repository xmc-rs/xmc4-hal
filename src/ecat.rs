pub trait EcatExt {
    fn constrain(self) -> Ecat;
}

impl EcatExt for Ecat {
    fn constrain(self) -> Ecat {
        Ecat {}
    }
}

pub struct Ecat {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}