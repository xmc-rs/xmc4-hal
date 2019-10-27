pub trait UsbExt {
    fn constrain(self) -> Usb;
}

impl UsbExt for Usb {
    fn constrain(self) -> Usb {
        Usb {}
    }
}

pub struct Usb {}

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
