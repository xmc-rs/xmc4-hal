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
