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
