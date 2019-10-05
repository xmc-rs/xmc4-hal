

pub trait PortExt {
    fn constrain(self) -> Port;
}

impl PortExt for Port {
    fn constrain(self) -> Port {
        Port {}
    }
}

pub struct Port {}

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
