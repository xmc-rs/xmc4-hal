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

    #[test]
    fn nothing() {
        // Do nothing test
    }
}
