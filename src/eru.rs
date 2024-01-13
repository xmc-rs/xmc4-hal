use crate::pac::{ERU0, ERU1};

pub trait EruExt {
    fn constrain(self) -> Eru;
}

impl EruExt for Eru {
    fn constrain(self) -> Eru {
        Eru {}
    }
}

pub enum Instance {
    Eru0,
    Eru1,
}

pub struct Eru {}

// IMPLEMENT PERIPHERAL AFTER THIS LINE

impl Eru {
    pub fn enable(&self) {}

    pub fn disable(&self) {}
}

#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
