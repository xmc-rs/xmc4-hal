//! GPIO ports.
//!
//! Split a PAC port into individually owned pins. Pin mode is tracked in the
//! type, matching the stm32-rs GPIO model and the XMC `IOCR` port-control field.

use core::convert::Infallible;
use core::marker::PhantomData;

use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};
use embedded_hal_02::digital::v2::{
    InputPin as InputPin02, OutputPin as OutputPin02, StatefulOutputPin as StatefulOutputPin02,
    ToggleableOutputPin as ToggleableOutputPin02,
};

use crate::Sealed;

/// Marker for pins in input configuration.
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (`PC = 0`).
pub struct Floating;
/// Pull-down input (`PC = 1`).
pub struct PullDown;
/// Pull-up input (`PC = 2`).
pub struct PullUp;

/// Marker for pins in output configuration.
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push-pull output.
pub struct PushPull;
/// Open-drain output.
pub struct OpenDrain;

/// Alternate output function `AF` (1 through 4).
pub struct Alternate<const AF: u8, MODE> {
    _mode: PhantomData<MODE>,
}

/// Digital pad disabled through `PDISC`.
pub struct Analog;

/// Implemented by input mode markers.
pub trait InputMode: Sealed {}
/// Implemented by output mode markers.
pub trait OutputMode: Sealed {}

impl Sealed for Floating {}
impl Sealed for PullDown {}
impl Sealed for PullUp {}
impl Sealed for PushPull {}
impl Sealed for OpenDrain {}
impl InputMode for Floating {}
impl InputMode for PullDown {}
impl InputMode for PullUp {}
impl OutputMode for PushPull {}
impl OutputMode for OpenDrain {}

/// Pad driver mode written to `Pn_PDR`.
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum OutputStrength {
    StrongSharp = 0,
    StrongMedium = 1,
    StrongSoft = 2,
    StrongSlow = 3,
    Medium = 4,
    Weak = 7,
}

/// Hardware port control selection (`HWSEL`).
#[derive(Copy, Clone, Debug)]
pub enum HwControl {
    Disabled,
    Peripheral1,
    Peripheral2,
}

/// Consume a PAC port and split it into pins.
pub trait PortExt: Sealed {
    /// Pin collection for this port.
    type Parts;
    /// Take ownership of the port and return its pins as floating inputs.
    fn split(self) -> Self::Parts;
}

macro_rules! write_pc {
    ($PORT:ident, $iocr:ident, $pc:ident, $valfn:ident, $bits:literal) => {
        unsafe {
            (*crate::pac::$PORT::ptr()).$iocr().modify(|_, w| {
                #[cfg(not(feature = "xmc4300"))]
                {
                    w.$pc().$valfn();
                }
                #[cfg(feature = "xmc4300")]
                unsafe {
                    w.$pc().bits($bits);
                }
                w
            });
        }
    };
}

macro_rules! pin_drive {
    ($PORT:ident, $P:ident, $pdr:ident, $pd:ident) => {
        impl<MODE> $P<MODE> {
            /// Pad driver strength.
            pub fn set_output_strength(&mut self, strength: OutputStrength) {
                unsafe {
                    (*crate::pac::$PORT::ptr()).$pdr().modify(|_, w| unsafe {
                        w.$pd().bits(strength as u8);
                        w
                    });
                }
            }
        }
    };
}

macro_rules! pin {
    ($PORT:ident, $P:ident, $n:literal, $iocr:ident, $pc:ident, $ps:ident, $pr:ident, $pinb:ident, $hw:ident) => {
        /// Port pin.
        pub struct $P<MODE> {
            _mode: PhantomData<MODE>,
        }

        impl<MODE> $P<MODE> {
            fn make_floating(&self) {
                write_pc!($PORT, $iocr, $pc, value1, 0);
            }

            fn read_input(&self) -> bool {
                unsafe {
                    (*crate::pac::$PORT::ptr())
                        .in_()
                        .read()
                        .$pinb()
                        .bit_is_set()
                }
            }

            fn read_output(&self) -> bool {
                unsafe {
                    (*crate::pac::$PORT::ptr())
                        .out()
                        .read()
                        .$pinb()
                        .bit_is_set()
                }
            }

            /// Floating input.
            pub fn into_floating_input(self) -> $P<Input<Floating>> {
                self.make_floating();
                $P { _mode: PhantomData }
            }

            /// Pull-down input.
            pub fn into_pull_down_input(self) -> $P<Input<PullDown>> {
                write_pc!($PORT, $iocr, $pc, value2, 1);
                $P { _mode: PhantomData }
            }

            /// Pull-up input.
            pub fn into_pull_up_input(self) -> $P<Input<PullUp>> {
                write_pc!($PORT, $iocr, $pc, value3, 2);
                $P { _mode: PhantomData }
            }

            /// Push-pull software output.
            pub fn into_push_pull_output(self) -> $P<Output<PushPull>> {
                write_pc!($PORT, $iocr, $pc, value9, 16);
                $P { _mode: PhantomData }
            }

            /// Open-drain software output.
            pub fn into_open_drain_output(self) -> $P<Output<OpenDrain>> {
                write_pc!($PORT, $iocr, $pc, value14, 24);
                $P { _mode: PhantomData }
            }

            /// Push-pull alternate function `AF` in 1..=4.
            pub fn into_alternate_push_pull<const AF: u8>(self) -> $P<Alternate<AF, PushPull>> {
                match AF {
                    1 => {
                        write_pc!($PORT, $iocr, $pc, value10, 17);
                    }
                    2 => {
                        write_pc!($PORT, $iocr, $pc, value11, 18);
                    }
                    3 => {
                        write_pc!($PORT, $iocr, $pc, value12, 19);
                    }
                    4 => {
                        write_pc!($PORT, $iocr, $pc, value13, 20);
                    }
                    _ => panic!("alternate function must be 1..=4"),
                }
                $P { _mode: PhantomData }
            }

            /// Open-drain alternate function `AF` in 1..=4.
            pub fn into_alternate_open_drain<const AF: u8>(self) -> $P<Alternate<AF, OpenDrain>> {
                match AF {
                    1 => {
                        write_pc!($PORT, $iocr, $pc, value15, 25);
                    }
                    2 => {
                        write_pc!($PORT, $iocr, $pc, value16, 26);
                    }
                    3 => {
                        write_pc!($PORT, $iocr, $pc, value17, 27);
                    }
                    4 => {
                        write_pc!($PORT, $iocr, $pc, value18, 28);
                    }
                    _ => panic!("alternate function must be 1..=4"),
                }
                $P { _mode: PhantomData }
            }

            /// Disable the digital pad. `PDISC` is read-only in the SVD, so this writes the register directly.
            pub fn into_analog(self) -> $P<Analog> {
                unsafe {
                    let reg = (*crate::pac::$PORT::ptr()).pdisc().as_ptr();
                    let value = core::ptr::read_volatile(reg);
                    core::ptr::write_volatile(reg, value | (1 << $n));
                }
                $P { _mode: PhantomData }
            }

            /// Re-enable the digital pad after [`Self::into_analog`].
            pub fn into_digital_floating(self) -> $P<Input<Floating>> {
                unsafe {
                    let reg = (*crate::pac::$PORT::ptr()).pdisc().as_ptr();
                    let value = core::ptr::read_volatile(reg);
                    core::ptr::write_volatile(reg, value & !(1 << $n));
                }
                self.into_floating_input()
            }

            /// Hardware control path selection.
            pub fn set_hw_control(&mut self, control: HwControl) {
                unsafe {
                    (*crate::pac::$PORT::ptr()).hwsel().modify(|_, w| {
                        #[cfg(not(feature = "xmc4300"))]
                        match control {
                            HwControl::Disabled => {
                                w.$hw().value1();
                            }
                            HwControl::Peripheral1 => {
                                w.$hw().value2();
                            }
                            HwControl::Peripheral2 => {
                                w.$hw().value3();
                            }
                        }
                        #[cfg(feature = "xmc4300")]
                        match control {
                            HwControl::Disabled => {
                                w.$hw().const_00();
                            }
                            HwControl::Peripheral1 => {
                                w.$hw().const_01();
                            }
                            HwControl::Peripheral2 => {
                                w.$hw().const_10();
                            }
                        }
                        w
                    });
                }
            }

            fn set_output_high(&mut self) {
                unsafe {
                    (*crate::pac::$PORT::ptr())
                        .omr()
                        .write(|w| w.$ps().set_bit());
                }
            }

            fn set_output_low(&mut self) {
                unsafe {
                    (*crate::pac::$PORT::ptr())
                        .omr()
                        .write(|w| w.$pr().set_bit());
                }
            }

            fn toggle_output(&mut self) {
                unsafe {
                    (*crate::pac::$PORT::ptr()).omr().write(|w| {
                        w.$ps().set_bit();
                        w.$pr().set_bit()
                    });
                }
            }
        }

        impl<MODE> ErrorType for $P<MODE> {
            type Error = Infallible;
        }

        impl<MODE: InputMode> InputPin for $P<Input<MODE>> {
            fn is_high(&mut self) -> Result<bool, Self::Error> {
                Ok(self.read_input())
            }
            fn is_low(&mut self) -> Result<bool, Self::Error> {
                Ok(!self.read_input())
            }
        }

        impl<MODE: OutputMode> OutputPin for $P<Output<MODE>> {
            fn set_low(&mut self) -> Result<(), Self::Error> {
                self.set_output_low();
                Ok(())
            }
            fn set_high(&mut self) -> Result<(), Self::Error> {
                self.set_output_high();
                Ok(())
            }
        }

        impl<MODE: OutputMode> StatefulOutputPin for $P<Output<MODE>> {
            fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                Ok(self.read_output())
            }
            fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                Ok(!self.read_output())
            }
            fn toggle(&mut self) -> Result<(), Self::Error> {
                self.toggle_output();
                Ok(())
            }
        }

        impl<MODE: InputMode> InputPin02 for $P<Input<MODE>> {
            type Error = Infallible;
            fn is_high(&self) -> Result<bool, Self::Error> {
                Ok(self.read_input())
            }
            fn is_low(&self) -> Result<bool, Self::Error> {
                Ok(!self.read_input())
            }
        }

        impl<MODE: OutputMode> OutputPin02 for $P<Output<MODE>> {
            type Error = Infallible;
            fn set_low(&mut self) -> Result<(), Self::Error> {
                self.set_output_low();
                Ok(())
            }
            fn set_high(&mut self) -> Result<(), Self::Error> {
                self.set_output_high();
                Ok(())
            }
        }

        impl<MODE: OutputMode> StatefulOutputPin02 for $P<Output<MODE>> {
            fn is_set_high(&self) -> Result<bool, Self::Error> {
                Ok(self.read_output())
            }
            fn is_set_low(&self) -> Result<bool, Self::Error> {
                Ok(!self.read_output())
            }
        }

        impl<MODE: OutputMode> ToggleableOutputPin02 for $P<Output<MODE>> {
            type Error = Infallible;
            fn toggle(&mut self) -> Result<(), Self::Error> {
                self.toggle_output();
                Ok(())
            }
        }
    };
}

#[cfg(any(feature = "xmc4100", feature = "xmc4200"))]
mod small {
    use super::*;

    /// Pins of `PORT0`.
    pub struct PORT0Parts {
        /// P0.0.
        pub p0_0: P0_0<Input<Floating>>,
        /// P0.1.
        pub p0_1: P0_1<Input<Floating>>,
        /// P0.2.
        pub p0_2: P0_2<Input<Floating>>,
        /// P0.3.
        pub p0_3: P0_3<Input<Floating>>,
        /// P0.4.
        pub p0_4: P0_4<Input<Floating>>,
        /// P0.5.
        pub p0_5: P0_5<Input<Floating>>,
        /// P0.6.
        pub p0_6: P0_6<Input<Floating>>,
        /// P0.7.
        pub p0_7: P0_7<Input<Floating>>,
        /// P0.8.
        pub p0_8: P0_8<Input<Floating>>,
        /// P0.9.
        pub p0_9: P0_9<Input<Floating>>,
        /// P0.10.
        pub p0_10: P0_10<Input<Floating>>,
        /// P0.11.
        pub p0_11: P0_11<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT0 {}

    impl PortExt for crate::pac::PORT0 {
        type Parts = PORT0Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT0Parts {
                p0_0: P0_0 { _mode: PhantomData },
                p0_1: P0_1 { _mode: PhantomData },
                p0_2: P0_2 { _mode: PhantomData },
                p0_3: P0_3 { _mode: PhantomData },
                p0_4: P0_4 { _mode: PhantomData },
                p0_5: P0_5 { _mode: PhantomData },
                p0_6: P0_6 { _mode: PhantomData },
                p0_7: P0_7 { _mode: PhantomData },
                p0_8: P0_8 { _mode: PhantomData },
                p0_9: P0_9 { _mode: PhantomData },
                p0_10: P0_10 { _mode: PhantomData },
                p0_11: P0_11 { _mode: PhantomData },
            };
            parts.p0_0.make_floating();
            parts.p0_1.make_floating();
            parts.p0_2.make_floating();
            parts.p0_3.make_floating();
            parts.p0_4.make_floating();
            parts.p0_5.make_floating();
            parts.p0_6.make_floating();
            parts.p0_7.make_floating();
            parts.p0_8.make_floating();
            parts.p0_9.make_floating();
            parts.p0_10.make_floating();
            parts.p0_11.make_floating();
            parts
        }
    }

    pin!(PORT0, P0_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT0, P0_0, pdr0, pd0);
    pin!(PORT0, P0_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT0, P0_1, pdr0, pd1);
    pin!(PORT0, P0_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT0, P0_2, pdr0, pd2);
    pin!(PORT0, P0_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT0, P0_3, pdr0, pd3);
    pin!(PORT0, P0_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT0, P0_4, pdr0, pd4);
    pin!(PORT0, P0_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT0, P0_5, pdr0, pd5);
    pin!(PORT0, P0_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT0, P0_6, pdr0, pd6);
    pin!(PORT0, P0_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT0, P0_7, pdr0, pd7);
    pin!(PORT0, P0_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT0, P0_8, pdr1, pd8);
    pin!(PORT0, P0_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT0, P0_9, pdr1, pd9);
    pin!(PORT0, P0_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT0, P0_10, pdr1, pd10);
    pin!(PORT0, P0_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT0, P0_11, pdr1, pd11);

    /// Pins of `PORT1`.
    pub struct PORT1Parts {
        /// P1.0.
        pub p1_0: P1_0<Input<Floating>>,
        /// P1.1.
        pub p1_1: P1_1<Input<Floating>>,
        /// P1.2.
        pub p1_2: P1_2<Input<Floating>>,
        /// P1.3.
        pub p1_3: P1_3<Input<Floating>>,
        /// P1.4.
        pub p1_4: P1_4<Input<Floating>>,
        /// P1.5.
        pub p1_5: P1_5<Input<Floating>>,
        /// P1.6.
        pub p1_6: P1_6<Input<Floating>>,
        /// P1.7.
        pub p1_7: P1_7<Input<Floating>>,
        /// P1.8.
        pub p1_8: P1_8<Input<Floating>>,
        /// P1.9.
        pub p1_9: P1_9<Input<Floating>>,
        /// P1.10.
        pub p1_10: P1_10<Input<Floating>>,
        /// P1.11.
        pub p1_11: P1_11<Input<Floating>>,
        /// P1.12.
        pub p1_12: P1_12<Input<Floating>>,
        /// P1.13.
        pub p1_13: P1_13<Input<Floating>>,
        /// P1.14.
        pub p1_14: P1_14<Input<Floating>>,
        /// P1.15.
        pub p1_15: P1_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT1 {}

    impl PortExt for crate::pac::PORT1 {
        type Parts = PORT1Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT1Parts {
                p1_0: P1_0 { _mode: PhantomData },
                p1_1: P1_1 { _mode: PhantomData },
                p1_2: P1_2 { _mode: PhantomData },
                p1_3: P1_3 { _mode: PhantomData },
                p1_4: P1_4 { _mode: PhantomData },
                p1_5: P1_5 { _mode: PhantomData },
                p1_6: P1_6 { _mode: PhantomData },
                p1_7: P1_7 { _mode: PhantomData },
                p1_8: P1_8 { _mode: PhantomData },
                p1_9: P1_9 { _mode: PhantomData },
                p1_10: P1_10 { _mode: PhantomData },
                p1_11: P1_11 { _mode: PhantomData },
                p1_12: P1_12 { _mode: PhantomData },
                p1_13: P1_13 { _mode: PhantomData },
                p1_14: P1_14 { _mode: PhantomData },
                p1_15: P1_15 { _mode: PhantomData },
            };
            parts.p1_0.make_floating();
            parts.p1_1.make_floating();
            parts.p1_2.make_floating();
            parts.p1_3.make_floating();
            parts.p1_4.make_floating();
            parts.p1_5.make_floating();
            parts.p1_6.make_floating();
            parts.p1_7.make_floating();
            parts.p1_8.make_floating();
            parts.p1_9.make_floating();
            parts.p1_10.make_floating();
            parts.p1_11.make_floating();
            parts.p1_12.make_floating();
            parts.p1_13.make_floating();
            parts.p1_14.make_floating();
            parts.p1_15.make_floating();
            parts
        }
    }

    pin!(PORT1, P1_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT1, P1_0, pdr0, pd0);
    pin!(PORT1, P1_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT1, P1_1, pdr0, pd1);
    pin!(PORT1, P1_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT1, P1_2, pdr0, pd2);
    pin!(PORT1, P1_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT1, P1_3, pdr0, pd3);
    pin!(PORT1, P1_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT1, P1_4, pdr0, pd4);
    pin!(PORT1, P1_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT1, P1_5, pdr0, pd5);
    pin!(PORT1, P1_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT1, P1_6, pdr0, pd6);
    pin!(PORT1, P1_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT1, P1_7, pdr0, pd7);
    pin!(PORT1, P1_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT1, P1_8, pdr1, pd8);
    pin!(PORT1, P1_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT1, P1_9, pdr1, pd9);
    pin!(PORT1, P1_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT1, P1_10, pdr1, pd10);
    pin!(PORT1, P1_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT1, P1_11, pdr1, pd11);
    pin!(PORT1, P1_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT1, P1_12, pdr1, pd12);
    pin!(PORT1, P1_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT1, P1_13, pdr1, pd13);
    pin!(PORT1, P1_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT1, P1_14, pdr1, pd14);
    pin!(PORT1, P1_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT1, P1_15, pdr1, pd15);

    /// Pins of `PORT2`.
    pub struct PORT2Parts {
        /// P2.0.
        pub p2_0: P2_0<Input<Floating>>,
        /// P2.1.
        pub p2_1: P2_1<Input<Floating>>,
        /// P2.2.
        pub p2_2: P2_2<Input<Floating>>,
        /// P2.3.
        pub p2_3: P2_3<Input<Floating>>,
        /// P2.4.
        pub p2_4: P2_4<Input<Floating>>,
        /// P2.5.
        pub p2_5: P2_5<Input<Floating>>,
        /// P2.6.
        pub p2_6: P2_6<Input<Floating>>,
        /// P2.7.
        pub p2_7: P2_7<Input<Floating>>,
        /// P2.8.
        pub p2_8: P2_8<Input<Floating>>,
        /// P2.9.
        pub p2_9: P2_9<Input<Floating>>,
        /// P2.10.
        pub p2_10: P2_10<Input<Floating>>,
        /// P2.11.
        pub p2_11: P2_11<Input<Floating>>,
        /// P2.12.
        pub p2_12: P2_12<Input<Floating>>,
        /// P2.13.
        pub p2_13: P2_13<Input<Floating>>,
        /// P2.14.
        pub p2_14: P2_14<Input<Floating>>,
        /// P2.15.
        pub p2_15: P2_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT2 {}

    impl PortExt for crate::pac::PORT2 {
        type Parts = PORT2Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT2Parts {
                p2_0: P2_0 { _mode: PhantomData },
                p2_1: P2_1 { _mode: PhantomData },
                p2_2: P2_2 { _mode: PhantomData },
                p2_3: P2_3 { _mode: PhantomData },
                p2_4: P2_4 { _mode: PhantomData },
                p2_5: P2_5 { _mode: PhantomData },
                p2_6: P2_6 { _mode: PhantomData },
                p2_7: P2_7 { _mode: PhantomData },
                p2_8: P2_8 { _mode: PhantomData },
                p2_9: P2_9 { _mode: PhantomData },
                p2_10: P2_10 { _mode: PhantomData },
                p2_11: P2_11 { _mode: PhantomData },
                p2_12: P2_12 { _mode: PhantomData },
                p2_13: P2_13 { _mode: PhantomData },
                p2_14: P2_14 { _mode: PhantomData },
                p2_15: P2_15 { _mode: PhantomData },
            };
            parts.p2_0.make_floating();
            parts.p2_1.make_floating();
            parts.p2_2.make_floating();
            parts.p2_3.make_floating();
            parts.p2_4.make_floating();
            parts.p2_5.make_floating();
            parts.p2_6.make_floating();
            parts.p2_7.make_floating();
            parts.p2_8.make_floating();
            parts.p2_9.make_floating();
            parts.p2_10.make_floating();
            parts.p2_11.make_floating();
            parts.p2_12.make_floating();
            parts.p2_13.make_floating();
            parts.p2_14.make_floating();
            parts.p2_15.make_floating();
            parts
        }
    }

    pin!(PORT2, P2_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT2, P2_0, pdr0, pd0);
    pin!(PORT2, P2_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT2, P2_1, pdr0, pd1);
    pin!(PORT2, P2_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT2, P2_2, pdr0, pd2);
    pin!(PORT2, P2_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT2, P2_3, pdr0, pd3);
    pin!(PORT2, P2_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT2, P2_4, pdr0, pd4);
    pin!(PORT2, P2_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT2, P2_5, pdr0, pd5);
    pin!(PORT2, P2_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT2, P2_6, pdr0, pd6);
    pin!(PORT2, P2_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT2, P2_7, pdr0, pd7);
    pin!(PORT2, P2_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT2, P2_8, pdr1, pd8);
    pin!(PORT2, P2_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT2, P2_9, pdr1, pd9);
    pin!(PORT2, P2_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT2, P2_10, pdr1, pd10);
    pin!(PORT2, P2_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT2, P2_11, pdr1, pd11);
    pin!(PORT2, P2_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT2, P2_12, pdr1, pd12);
    pin!(PORT2, P2_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT2, P2_13, pdr1, pd13);
    pin!(PORT2, P2_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT2, P2_14, pdr1, pd14);
    pin!(PORT2, P2_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT2, P2_15, pdr1, pd15);

    /// Pins of `PORT3`.
    pub struct PORT3Parts {
        /// P3.0.
        pub p3_0: P3_0<Input<Floating>>,
        /// P3.1.
        pub p3_1: P3_1<Input<Floating>>,
        /// P3.2.
        pub p3_2: P3_2<Input<Floating>>,
        /// P3.3.
        pub p3_3: P3_3<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT3 {}

    impl PortExt for crate::pac::PORT3 {
        type Parts = PORT3Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT3Parts {
                p3_0: P3_0 { _mode: PhantomData },
                p3_1: P3_1 { _mode: PhantomData },
                p3_2: P3_2 { _mode: PhantomData },
                p3_3: P3_3 { _mode: PhantomData },
            };
            parts.p3_0.make_floating();
            parts.p3_1.make_floating();
            parts.p3_2.make_floating();
            parts.p3_3.make_floating();
            parts
        }
    }

    pin!(PORT3, P3_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT3, P3_0, pdr0, pd0);
    pin!(PORT3, P3_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT3, P3_1, pdr0, pd1);
    pin!(PORT3, P3_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT3, P3_2, pdr0, pd2);
    pin!(PORT3, P3_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT3, P3_3, pdr0, pd3);

    /// Pins of `PORT14`.
    pub struct PORT14Parts {
        /// P14.0.
        pub p14_0: P14_0<Input<Floating>>,
        /// P14.1.
        pub p14_1: P14_1<Input<Floating>>,
        /// P14.2.
        pub p14_2: P14_2<Input<Floating>>,
        /// P14.3.
        pub p14_3: P14_3<Input<Floating>>,
        /// P14.4.
        pub p14_4: P14_4<Input<Floating>>,
        /// P14.5.
        pub p14_5: P14_5<Input<Floating>>,
        /// P14.6.
        pub p14_6: P14_6<Input<Floating>>,
        /// P14.7.
        pub p14_7: P14_7<Input<Floating>>,
        /// P14.8.
        pub p14_8: P14_8<Input<Floating>>,
        /// P14.9.
        pub p14_9: P14_9<Input<Floating>>,
        /// P14.10.
        pub p14_10: P14_10<Input<Floating>>,
        /// P14.11.
        pub p14_11: P14_11<Input<Floating>>,
        /// P14.12.
        pub p14_12: P14_12<Input<Floating>>,
        /// P14.13.
        pub p14_13: P14_13<Input<Floating>>,
        /// P14.14.
        pub p14_14: P14_14<Input<Floating>>,
        /// P14.15.
        pub p14_15: P14_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT14 {}

    impl PortExt for crate::pac::PORT14 {
        type Parts = PORT14Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT14Parts {
                p14_0: P14_0 { _mode: PhantomData },
                p14_1: P14_1 { _mode: PhantomData },
                p14_2: P14_2 { _mode: PhantomData },
                p14_3: P14_3 { _mode: PhantomData },
                p14_4: P14_4 { _mode: PhantomData },
                p14_5: P14_5 { _mode: PhantomData },
                p14_6: P14_6 { _mode: PhantomData },
                p14_7: P14_7 { _mode: PhantomData },
                p14_8: P14_8 { _mode: PhantomData },
                p14_9: P14_9 { _mode: PhantomData },
                p14_10: P14_10 { _mode: PhantomData },
                p14_11: P14_11 { _mode: PhantomData },
                p14_12: P14_12 { _mode: PhantomData },
                p14_13: P14_13 { _mode: PhantomData },
                p14_14: P14_14 { _mode: PhantomData },
                p14_15: P14_15 { _mode: PhantomData },
            };
            parts.p14_0.make_floating();
            parts.p14_1.make_floating();
            parts.p14_2.make_floating();
            parts.p14_3.make_floating();
            parts.p14_4.make_floating();
            parts.p14_5.make_floating();
            parts.p14_6.make_floating();
            parts.p14_7.make_floating();
            parts.p14_8.make_floating();
            parts.p14_9.make_floating();
            parts.p14_10.make_floating();
            parts.p14_11.make_floating();
            parts.p14_12.make_floating();
            parts.p14_13.make_floating();
            parts.p14_14.make_floating();
            parts.p14_15.make_floating();
            parts
        }
    }

    pin!(PORT14, P14_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin!(PORT14, P14_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin!(PORT14, P14_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin!(PORT14, P14_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin!(PORT14, P14_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin!(PORT14, P14_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin!(PORT14, P14_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin!(PORT14, P14_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin!(PORT14, P14_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin!(PORT14, P14_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin!(PORT14, P14_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin!(PORT14, P14_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin!(PORT14, P14_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin!(PORT14, P14_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin!(PORT14, P14_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin!(PORT14, P14_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
}

#[cfg(any(feature = "xmc4100", feature = "xmc4200"))]
pub use small::*;

#[cfg(any(feature = "xmc4300", feature = "xmc4400"))]
mod mid {
    use super::*;

    /// Pins of `PORT0`.
    pub struct PORT0Parts {
        /// P0.0.
        pub p0_0: P0_0<Input<Floating>>,
        /// P0.1.
        pub p0_1: P0_1<Input<Floating>>,
        /// P0.2.
        pub p0_2: P0_2<Input<Floating>>,
        /// P0.3.
        pub p0_3: P0_3<Input<Floating>>,
        /// P0.4.
        pub p0_4: P0_4<Input<Floating>>,
        /// P0.5.
        pub p0_5: P0_5<Input<Floating>>,
        /// P0.6.
        pub p0_6: P0_6<Input<Floating>>,
        /// P0.7.
        pub p0_7: P0_7<Input<Floating>>,
        /// P0.8.
        pub p0_8: P0_8<Input<Floating>>,
        /// P0.9.
        pub p0_9: P0_9<Input<Floating>>,
        /// P0.10.
        pub p0_10: P0_10<Input<Floating>>,
        /// P0.11.
        pub p0_11: P0_11<Input<Floating>>,
        /// P0.12.
        pub p0_12: P0_12<Input<Floating>>,
        /// P0.13.
        pub p0_13: P0_13<Input<Floating>>,
        /// P0.14.
        pub p0_14: P0_14<Input<Floating>>,
        /// P0.15.
        pub p0_15: P0_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT0 {}

    impl PortExt for crate::pac::PORT0 {
        type Parts = PORT0Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT0Parts {
                p0_0: P0_0 { _mode: PhantomData },
                p0_1: P0_1 { _mode: PhantomData },
                p0_2: P0_2 { _mode: PhantomData },
                p0_3: P0_3 { _mode: PhantomData },
                p0_4: P0_4 { _mode: PhantomData },
                p0_5: P0_5 { _mode: PhantomData },
                p0_6: P0_6 { _mode: PhantomData },
                p0_7: P0_7 { _mode: PhantomData },
                p0_8: P0_8 { _mode: PhantomData },
                p0_9: P0_9 { _mode: PhantomData },
                p0_10: P0_10 { _mode: PhantomData },
                p0_11: P0_11 { _mode: PhantomData },
                p0_12: P0_12 { _mode: PhantomData },
                p0_13: P0_13 { _mode: PhantomData },
                p0_14: P0_14 { _mode: PhantomData },
                p0_15: P0_15 { _mode: PhantomData },
            };
            parts.p0_0.make_floating();
            parts.p0_1.make_floating();
            parts.p0_2.make_floating();
            parts.p0_3.make_floating();
            parts.p0_4.make_floating();
            parts.p0_5.make_floating();
            parts.p0_6.make_floating();
            parts.p0_7.make_floating();
            parts.p0_8.make_floating();
            parts.p0_9.make_floating();
            parts.p0_10.make_floating();
            parts.p0_11.make_floating();
            parts.p0_12.make_floating();
            parts.p0_13.make_floating();
            parts.p0_14.make_floating();
            parts.p0_15.make_floating();
            parts
        }
    }

    pin!(PORT0, P0_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT0, P0_0, pdr0, pd0);
    pin!(PORT0, P0_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT0, P0_1, pdr0, pd1);
    pin!(PORT0, P0_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT0, P0_2, pdr0, pd2);
    pin!(PORT0, P0_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT0, P0_3, pdr0, pd3);
    pin!(PORT0, P0_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT0, P0_4, pdr0, pd4);
    pin!(PORT0, P0_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT0, P0_5, pdr0, pd5);
    pin!(PORT0, P0_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT0, P0_6, pdr0, pd6);
    pin!(PORT0, P0_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT0, P0_7, pdr0, pd7);
    pin!(PORT0, P0_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT0, P0_8, pdr1, pd8);
    pin!(PORT0, P0_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT0, P0_9, pdr1, pd9);
    pin!(PORT0, P0_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT0, P0_10, pdr1, pd10);
    pin!(PORT0, P0_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT0, P0_11, pdr1, pd11);
    pin!(PORT0, P0_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT0, P0_12, pdr1, pd12);
    pin!(PORT0, P0_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT0, P0_13, pdr1, pd13);
    pin!(PORT0, P0_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT0, P0_14, pdr1, pd14);
    pin!(PORT0, P0_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT0, P0_15, pdr1, pd15);

    /// Pins of `PORT1`.
    pub struct PORT1Parts {
        /// P1.0.
        pub p1_0: P1_0<Input<Floating>>,
        /// P1.1.
        pub p1_1: P1_1<Input<Floating>>,
        /// P1.2.
        pub p1_2: P1_2<Input<Floating>>,
        /// P1.3.
        pub p1_3: P1_3<Input<Floating>>,
        /// P1.4.
        pub p1_4: P1_4<Input<Floating>>,
        /// P1.5.
        pub p1_5: P1_5<Input<Floating>>,
        /// P1.6.
        pub p1_6: P1_6<Input<Floating>>,
        /// P1.7.
        pub p1_7: P1_7<Input<Floating>>,
        /// P1.8.
        pub p1_8: P1_8<Input<Floating>>,
        /// P1.9.
        pub p1_9: P1_9<Input<Floating>>,
        /// P1.10.
        pub p1_10: P1_10<Input<Floating>>,
        /// P1.11.
        pub p1_11: P1_11<Input<Floating>>,
        /// P1.12.
        pub p1_12: P1_12<Input<Floating>>,
        /// P1.13.
        pub p1_13: P1_13<Input<Floating>>,
        /// P1.14.
        pub p1_14: P1_14<Input<Floating>>,
        /// P1.15.
        pub p1_15: P1_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT1 {}

    impl PortExt for crate::pac::PORT1 {
        type Parts = PORT1Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT1Parts {
                p1_0: P1_0 { _mode: PhantomData },
                p1_1: P1_1 { _mode: PhantomData },
                p1_2: P1_2 { _mode: PhantomData },
                p1_3: P1_3 { _mode: PhantomData },
                p1_4: P1_4 { _mode: PhantomData },
                p1_5: P1_5 { _mode: PhantomData },
                p1_6: P1_6 { _mode: PhantomData },
                p1_7: P1_7 { _mode: PhantomData },
                p1_8: P1_8 { _mode: PhantomData },
                p1_9: P1_9 { _mode: PhantomData },
                p1_10: P1_10 { _mode: PhantomData },
                p1_11: P1_11 { _mode: PhantomData },
                p1_12: P1_12 { _mode: PhantomData },
                p1_13: P1_13 { _mode: PhantomData },
                p1_14: P1_14 { _mode: PhantomData },
                p1_15: P1_15 { _mode: PhantomData },
            };
            parts.p1_0.make_floating();
            parts.p1_1.make_floating();
            parts.p1_2.make_floating();
            parts.p1_3.make_floating();
            parts.p1_4.make_floating();
            parts.p1_5.make_floating();
            parts.p1_6.make_floating();
            parts.p1_7.make_floating();
            parts.p1_8.make_floating();
            parts.p1_9.make_floating();
            parts.p1_10.make_floating();
            parts.p1_11.make_floating();
            parts.p1_12.make_floating();
            parts.p1_13.make_floating();
            parts.p1_14.make_floating();
            parts.p1_15.make_floating();
            parts
        }
    }

    pin!(PORT1, P1_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT1, P1_0, pdr0, pd0);
    pin!(PORT1, P1_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT1, P1_1, pdr0, pd1);
    pin!(PORT1, P1_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT1, P1_2, pdr0, pd2);
    pin!(PORT1, P1_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT1, P1_3, pdr0, pd3);
    pin!(PORT1, P1_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT1, P1_4, pdr0, pd4);
    pin!(PORT1, P1_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT1, P1_5, pdr0, pd5);
    pin!(PORT1, P1_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT1, P1_6, pdr0, pd6);
    pin!(PORT1, P1_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT1, P1_7, pdr0, pd7);
    pin!(PORT1, P1_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT1, P1_8, pdr1, pd8);
    pin!(PORT1, P1_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT1, P1_9, pdr1, pd9);
    pin!(PORT1, P1_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT1, P1_10, pdr1, pd10);
    pin!(PORT1, P1_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT1, P1_11, pdr1, pd11);
    pin!(PORT1, P1_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT1, P1_12, pdr1, pd12);
    pin!(PORT1, P1_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT1, P1_13, pdr1, pd13);
    pin!(PORT1, P1_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT1, P1_14, pdr1, pd14);
    pin!(PORT1, P1_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT1, P1_15, pdr1, pd15);

    /// Pins of `PORT2`.
    pub struct PORT2Parts {
        /// P2.0.
        pub p2_0: P2_0<Input<Floating>>,
        /// P2.1.
        pub p2_1: P2_1<Input<Floating>>,
        /// P2.2.
        pub p2_2: P2_2<Input<Floating>>,
        /// P2.3.
        pub p2_3: P2_3<Input<Floating>>,
        /// P2.4.
        pub p2_4: P2_4<Input<Floating>>,
        /// P2.5.
        pub p2_5: P2_5<Input<Floating>>,
        /// P2.6.
        pub p2_6: P2_6<Input<Floating>>,
        /// P2.7.
        pub p2_7: P2_7<Input<Floating>>,
        /// P2.8.
        pub p2_8: P2_8<Input<Floating>>,
        /// P2.9.
        pub p2_9: P2_9<Input<Floating>>,
        /// P2.10.
        pub p2_10: P2_10<Input<Floating>>,
        /// P2.11.
        pub p2_11: P2_11<Input<Floating>>,
        /// P2.12.
        pub p2_12: P2_12<Input<Floating>>,
        /// P2.13.
        pub p2_13: P2_13<Input<Floating>>,
        /// P2.14.
        pub p2_14: P2_14<Input<Floating>>,
        /// P2.15.
        pub p2_15: P2_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT2 {}

    impl PortExt for crate::pac::PORT2 {
        type Parts = PORT2Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT2Parts {
                p2_0: P2_0 { _mode: PhantomData },
                p2_1: P2_1 { _mode: PhantomData },
                p2_2: P2_2 { _mode: PhantomData },
                p2_3: P2_3 { _mode: PhantomData },
                p2_4: P2_4 { _mode: PhantomData },
                p2_5: P2_5 { _mode: PhantomData },
                p2_6: P2_6 { _mode: PhantomData },
                p2_7: P2_7 { _mode: PhantomData },
                p2_8: P2_8 { _mode: PhantomData },
                p2_9: P2_9 { _mode: PhantomData },
                p2_10: P2_10 { _mode: PhantomData },
                p2_11: P2_11 { _mode: PhantomData },
                p2_12: P2_12 { _mode: PhantomData },
                p2_13: P2_13 { _mode: PhantomData },
                p2_14: P2_14 { _mode: PhantomData },
                p2_15: P2_15 { _mode: PhantomData },
            };
            parts.p2_0.make_floating();
            parts.p2_1.make_floating();
            parts.p2_2.make_floating();
            parts.p2_3.make_floating();
            parts.p2_4.make_floating();
            parts.p2_5.make_floating();
            parts.p2_6.make_floating();
            parts.p2_7.make_floating();
            parts.p2_8.make_floating();
            parts.p2_9.make_floating();
            parts.p2_10.make_floating();
            parts.p2_11.make_floating();
            parts.p2_12.make_floating();
            parts.p2_13.make_floating();
            parts.p2_14.make_floating();
            parts.p2_15.make_floating();
            parts
        }
    }

    pin!(PORT2, P2_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT2, P2_0, pdr0, pd0);
    pin!(PORT2, P2_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT2, P2_1, pdr0, pd1);
    pin!(PORT2, P2_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT2, P2_2, pdr0, pd2);
    pin!(PORT2, P2_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT2, P2_3, pdr0, pd3);
    pin!(PORT2, P2_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT2, P2_4, pdr0, pd4);
    pin!(PORT2, P2_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT2, P2_5, pdr0, pd5);
    pin!(PORT2, P2_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT2, P2_6, pdr0, pd6);
    pin!(PORT2, P2_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT2, P2_7, pdr0, pd7);
    pin!(PORT2, P2_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT2, P2_8, pdr1, pd8);
    pin!(PORT2, P2_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT2, P2_9, pdr1, pd9);
    pin!(PORT2, P2_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT2, P2_10, pdr1, pd10);
    pin!(PORT2, P2_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT2, P2_11, pdr1, pd11);
    pin!(PORT2, P2_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT2, P2_12, pdr1, pd12);
    pin!(PORT2, P2_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT2, P2_13, pdr1, pd13);
    pin!(PORT2, P2_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT2, P2_14, pdr1, pd14);
    pin!(PORT2, P2_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT2, P2_15, pdr1, pd15);

    /// Pins of `PORT3`.
    pub struct PORT3Parts {
        /// P3.0.
        pub p3_0: P3_0<Input<Floating>>,
        /// P3.1.
        pub p3_1: P3_1<Input<Floating>>,
        /// P3.2.
        pub p3_2: P3_2<Input<Floating>>,
        /// P3.3.
        pub p3_3: P3_3<Input<Floating>>,
        /// P3.4.
        pub p3_4: P3_4<Input<Floating>>,
        /// P3.5.
        pub p3_5: P3_5<Input<Floating>>,
        /// P3.6.
        pub p3_6: P3_6<Input<Floating>>,
        /// P3.7.
        pub p3_7: P3_7<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT3 {}

    impl PortExt for crate::pac::PORT3 {
        type Parts = PORT3Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT3Parts {
                p3_0: P3_0 { _mode: PhantomData },
                p3_1: P3_1 { _mode: PhantomData },
                p3_2: P3_2 { _mode: PhantomData },
                p3_3: P3_3 { _mode: PhantomData },
                p3_4: P3_4 { _mode: PhantomData },
                p3_5: P3_5 { _mode: PhantomData },
                p3_6: P3_6 { _mode: PhantomData },
                p3_7: P3_7 { _mode: PhantomData },
            };
            parts.p3_0.make_floating();
            parts.p3_1.make_floating();
            parts.p3_2.make_floating();
            parts.p3_3.make_floating();
            parts.p3_4.make_floating();
            parts.p3_5.make_floating();
            parts.p3_6.make_floating();
            parts.p3_7.make_floating();
            parts
        }
    }

    pin!(PORT3, P3_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT3, P3_0, pdr0, pd0);
    pin!(PORT3, P3_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT3, P3_1, pdr0, pd1);
    pin!(PORT3, P3_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT3, P3_2, pdr0, pd2);
    pin!(PORT3, P3_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT3, P3_3, pdr0, pd3);
    pin!(PORT3, P3_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT3, P3_4, pdr0, pd4);
    pin!(PORT3, P3_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT3, P3_5, pdr0, pd5);
    pin!(PORT3, P3_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT3, P3_6, pdr0, pd6);
    pin!(PORT3, P3_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT3, P3_7, pdr0, pd7);

    /// Pins of `PORT4`.
    pub struct PORT4Parts {
        /// P4.0.
        pub p4_0: P4_0<Input<Floating>>,
        /// P4.1.
        pub p4_1: P4_1<Input<Floating>>,
        /// P4.2.
        pub p4_2: P4_2<Input<Floating>>,
        /// P4.3.
        pub p4_3: P4_3<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT4 {}

    impl PortExt for crate::pac::PORT4 {
        type Parts = PORT4Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT4Parts {
                p4_0: P4_0 { _mode: PhantomData },
                p4_1: P4_1 { _mode: PhantomData },
                p4_2: P4_2 { _mode: PhantomData },
                p4_3: P4_3 { _mode: PhantomData },
            };
            parts.p4_0.make_floating();
            parts.p4_1.make_floating();
            parts.p4_2.make_floating();
            parts.p4_3.make_floating();
            parts
        }
    }

    pin!(PORT4, P4_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT4, P4_0, pdr0, pd0);
    pin!(PORT4, P4_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT4, P4_1, pdr0, pd1);
    pin!(PORT4, P4_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT4, P4_2, pdr0, pd2);
    pin!(PORT4, P4_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT4, P4_3, pdr0, pd3);

    /// Pins of `PORT5`.
    pub struct PORT5Parts {
        /// P5.0.
        pub p5_0: P5_0<Input<Floating>>,
        /// P5.1.
        pub p5_1: P5_1<Input<Floating>>,
        /// P5.2.
        pub p5_2: P5_2<Input<Floating>>,
        /// P5.3.
        pub p5_3: P5_3<Input<Floating>>,
        /// P5.4.
        pub p5_4: P5_4<Input<Floating>>,
        /// P5.5.
        pub p5_5: P5_5<Input<Floating>>,
        /// P5.6.
        pub p5_6: P5_6<Input<Floating>>,
        /// P5.7.
        pub p5_7: P5_7<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT5 {}

    impl PortExt for crate::pac::PORT5 {
        type Parts = PORT5Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT5Parts {
                p5_0: P5_0 { _mode: PhantomData },
                p5_1: P5_1 { _mode: PhantomData },
                p5_2: P5_2 { _mode: PhantomData },
                p5_3: P5_3 { _mode: PhantomData },
                p5_4: P5_4 { _mode: PhantomData },
                p5_5: P5_5 { _mode: PhantomData },
                p5_6: P5_6 { _mode: PhantomData },
                p5_7: P5_7 { _mode: PhantomData },
            };
            parts.p5_0.make_floating();
            parts.p5_1.make_floating();
            parts.p5_2.make_floating();
            parts.p5_3.make_floating();
            parts.p5_4.make_floating();
            parts.p5_5.make_floating();
            parts.p5_6.make_floating();
            parts.p5_7.make_floating();
            parts
        }
    }

    pin!(PORT5, P5_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT5, P5_0, pdr0, pd0);
    pin!(PORT5, P5_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT5, P5_1, pdr0, pd1);
    pin!(PORT5, P5_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT5, P5_2, pdr0, pd2);
    pin!(PORT5, P5_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT5, P5_3, pdr0, pd3);
    pin!(PORT5, P5_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT5, P5_4, pdr0, pd4);
    pin!(PORT5, P5_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT5, P5_5, pdr0, pd5);
    pin!(PORT5, P5_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT5, P5_6, pdr0, pd6);
    pin!(PORT5, P5_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT5, P5_7, pdr0, pd7);

    /// Pins of `PORT14`.
    pub struct PORT14Parts {
        /// P14.0.
        pub p14_0: P14_0<Input<Floating>>,
        /// P14.1.
        pub p14_1: P14_1<Input<Floating>>,
        /// P14.2.
        pub p14_2: P14_2<Input<Floating>>,
        /// P14.3.
        pub p14_3: P14_3<Input<Floating>>,
        /// P14.4.
        pub p14_4: P14_4<Input<Floating>>,
        /// P14.5.
        pub p14_5: P14_5<Input<Floating>>,
        /// P14.6.
        pub p14_6: P14_6<Input<Floating>>,
        /// P14.7.
        pub p14_7: P14_7<Input<Floating>>,
        /// P14.8.
        pub p14_8: P14_8<Input<Floating>>,
        /// P14.9.
        pub p14_9: P14_9<Input<Floating>>,
        /// P14.10.
        pub p14_10: P14_10<Input<Floating>>,
        /// P14.11.
        pub p14_11: P14_11<Input<Floating>>,
        /// P14.12.
        pub p14_12: P14_12<Input<Floating>>,
        /// P14.13.
        pub p14_13: P14_13<Input<Floating>>,
        /// P14.14.
        pub p14_14: P14_14<Input<Floating>>,
        /// P14.15.
        pub p14_15: P14_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT14 {}

    impl PortExt for crate::pac::PORT14 {
        type Parts = PORT14Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT14Parts {
                p14_0: P14_0 { _mode: PhantomData },
                p14_1: P14_1 { _mode: PhantomData },
                p14_2: P14_2 { _mode: PhantomData },
                p14_3: P14_3 { _mode: PhantomData },
                p14_4: P14_4 { _mode: PhantomData },
                p14_5: P14_5 { _mode: PhantomData },
                p14_6: P14_6 { _mode: PhantomData },
                p14_7: P14_7 { _mode: PhantomData },
                p14_8: P14_8 { _mode: PhantomData },
                p14_9: P14_9 { _mode: PhantomData },
                p14_10: P14_10 { _mode: PhantomData },
                p14_11: P14_11 { _mode: PhantomData },
                p14_12: P14_12 { _mode: PhantomData },
                p14_13: P14_13 { _mode: PhantomData },
                p14_14: P14_14 { _mode: PhantomData },
                p14_15: P14_15 { _mode: PhantomData },
            };
            parts.p14_0.make_floating();
            parts.p14_1.make_floating();
            parts.p14_2.make_floating();
            parts.p14_3.make_floating();
            parts.p14_4.make_floating();
            parts.p14_5.make_floating();
            parts.p14_6.make_floating();
            parts.p14_7.make_floating();
            parts.p14_8.make_floating();
            parts.p14_9.make_floating();
            parts.p14_10.make_floating();
            parts.p14_11.make_floating();
            parts.p14_12.make_floating();
            parts.p14_13.make_floating();
            parts.p14_14.make_floating();
            parts.p14_15.make_floating();
            parts
        }
    }

    pin!(PORT14, P14_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin!(PORT14, P14_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin!(PORT14, P14_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin!(PORT14, P14_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin!(PORT14, P14_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin!(PORT14, P14_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin!(PORT14, P14_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin!(PORT14, P14_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin!(PORT14, P14_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin!(PORT14, P14_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin!(PORT14, P14_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin!(PORT14, P14_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin!(PORT14, P14_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin!(PORT14, P14_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin!(PORT14, P14_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin!(PORT14, P14_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);

    /// Pins of `PORT15`.
    pub struct PORT15Parts {
        /// P15.0.
        pub p15_0: P15_0<Input<Floating>>,
        /// P15.1.
        pub p15_1: P15_1<Input<Floating>>,
        /// P15.2.
        pub p15_2: P15_2<Input<Floating>>,
        /// P15.3.
        pub p15_3: P15_3<Input<Floating>>,
        /// P15.4.
        pub p15_4: P15_4<Input<Floating>>,
        /// P15.5.
        pub p15_5: P15_5<Input<Floating>>,
        /// P15.6.
        pub p15_6: P15_6<Input<Floating>>,
        /// P15.7.
        pub p15_7: P15_7<Input<Floating>>,
        /// P15.8.
        pub p15_8: P15_8<Input<Floating>>,
        /// P15.9.
        pub p15_9: P15_9<Input<Floating>>,
        /// P15.10.
        pub p15_10: P15_10<Input<Floating>>,
        /// P15.11.
        pub p15_11: P15_11<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT15 {}

    impl PortExt for crate::pac::PORT15 {
        type Parts = PORT15Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT15Parts {
                p15_0: P15_0 { _mode: PhantomData },
                p15_1: P15_1 { _mode: PhantomData },
                p15_2: P15_2 { _mode: PhantomData },
                p15_3: P15_3 { _mode: PhantomData },
                p15_4: P15_4 { _mode: PhantomData },
                p15_5: P15_5 { _mode: PhantomData },
                p15_6: P15_6 { _mode: PhantomData },
                p15_7: P15_7 { _mode: PhantomData },
                p15_8: P15_8 { _mode: PhantomData },
                p15_9: P15_9 { _mode: PhantomData },
                p15_10: P15_10 { _mode: PhantomData },
                p15_11: P15_11 { _mode: PhantomData },
            };
            parts.p15_0.make_floating();
            parts.p15_1.make_floating();
            parts.p15_2.make_floating();
            parts.p15_3.make_floating();
            parts.p15_4.make_floating();
            parts.p15_5.make_floating();
            parts.p15_6.make_floating();
            parts.p15_7.make_floating();
            parts.p15_8.make_floating();
            parts.p15_9.make_floating();
            parts.p15_10.make_floating();
            parts.p15_11.make_floating();
            parts
        }
    }

    pin!(PORT15, P15_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin!(PORT15, P15_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin!(PORT15, P15_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin!(PORT15, P15_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin!(PORT15, P15_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin!(PORT15, P15_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin!(PORT15, P15_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin!(PORT15, P15_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin!(PORT15, P15_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin!(PORT15, P15_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin!(PORT15, P15_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin!(PORT15, P15_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
}

#[cfg(any(feature = "xmc4300", feature = "xmc4400"))]
pub use mid::*;

#[cfg(any(feature = "xmc4500"))]
mod xmc4500 {
    use super::*;

    /// Pins of `PORT0`.
    pub struct PORT0Parts {
        /// P0.0.
        pub p0_0: P0_0<Input<Floating>>,
        /// P0.1.
        pub p0_1: P0_1<Input<Floating>>,
        /// P0.2.
        pub p0_2: P0_2<Input<Floating>>,
        /// P0.3.
        pub p0_3: P0_3<Input<Floating>>,
        /// P0.4.
        pub p0_4: P0_4<Input<Floating>>,
        /// P0.5.
        pub p0_5: P0_5<Input<Floating>>,
        /// P0.6.
        pub p0_6: P0_6<Input<Floating>>,
        /// P0.7.
        pub p0_7: P0_7<Input<Floating>>,
        /// P0.8.
        pub p0_8: P0_8<Input<Floating>>,
        /// P0.9.
        pub p0_9: P0_9<Input<Floating>>,
        /// P0.10.
        pub p0_10: P0_10<Input<Floating>>,
        /// P0.11.
        pub p0_11: P0_11<Input<Floating>>,
        /// P0.12.
        pub p0_12: P0_12<Input<Floating>>,
        /// P0.13.
        pub p0_13: P0_13<Input<Floating>>,
        /// P0.14.
        pub p0_14: P0_14<Input<Floating>>,
        /// P0.15.
        pub p0_15: P0_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT0 {}

    impl PortExt for crate::pac::PORT0 {
        type Parts = PORT0Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT0Parts {
                p0_0: P0_0 { _mode: PhantomData },
                p0_1: P0_1 { _mode: PhantomData },
                p0_2: P0_2 { _mode: PhantomData },
                p0_3: P0_3 { _mode: PhantomData },
                p0_4: P0_4 { _mode: PhantomData },
                p0_5: P0_5 { _mode: PhantomData },
                p0_6: P0_6 { _mode: PhantomData },
                p0_7: P0_7 { _mode: PhantomData },
                p0_8: P0_8 { _mode: PhantomData },
                p0_9: P0_9 { _mode: PhantomData },
                p0_10: P0_10 { _mode: PhantomData },
                p0_11: P0_11 { _mode: PhantomData },
                p0_12: P0_12 { _mode: PhantomData },
                p0_13: P0_13 { _mode: PhantomData },
                p0_14: P0_14 { _mode: PhantomData },
                p0_15: P0_15 { _mode: PhantomData },
            };
            parts.p0_0.make_floating();
            parts.p0_1.make_floating();
            parts.p0_2.make_floating();
            parts.p0_3.make_floating();
            parts.p0_4.make_floating();
            parts.p0_5.make_floating();
            parts.p0_6.make_floating();
            parts.p0_7.make_floating();
            parts.p0_8.make_floating();
            parts.p0_9.make_floating();
            parts.p0_10.make_floating();
            parts.p0_11.make_floating();
            parts.p0_12.make_floating();
            parts.p0_13.make_floating();
            parts.p0_14.make_floating();
            parts.p0_15.make_floating();
            parts
        }
    }

    pin!(PORT0, P0_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT0, P0_0, pdr0, pd0);
    pin!(PORT0, P0_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT0, P0_1, pdr0, pd1);
    pin!(PORT0, P0_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT0, P0_2, pdr0, pd2);
    pin!(PORT0, P0_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT0, P0_3, pdr0, pd3);
    pin!(PORT0, P0_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT0, P0_4, pdr0, pd4);
    pin!(PORT0, P0_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT0, P0_5, pdr0, pd5);
    pin!(PORT0, P0_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT0, P0_6, pdr0, pd6);
    pin!(PORT0, P0_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT0, P0_7, pdr0, pd7);
    pin!(PORT0, P0_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT0, P0_8, pdr1, pd8);
    pin!(PORT0, P0_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT0, P0_9, pdr1, pd9);
    pin!(PORT0, P0_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT0, P0_10, pdr1, pd10);
    pin!(PORT0, P0_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT0, P0_11, pdr1, pd11);
    pin!(PORT0, P0_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT0, P0_12, pdr1, pd12);
    pin!(PORT0, P0_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT0, P0_13, pdr1, pd13);
    pin!(PORT0, P0_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT0, P0_14, pdr1, pd14);
    pin!(PORT0, P0_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT0, P0_15, pdr1, pd15);

    /// Pins of `PORT1`.
    pub struct PORT1Parts {
        /// P1.0.
        pub p1_0: P1_0<Input<Floating>>,
        /// P1.1.
        pub p1_1: P1_1<Input<Floating>>,
        /// P1.2.
        pub p1_2: P1_2<Input<Floating>>,
        /// P1.3.
        pub p1_3: P1_3<Input<Floating>>,
        /// P1.4.
        pub p1_4: P1_4<Input<Floating>>,
        /// P1.5.
        pub p1_5: P1_5<Input<Floating>>,
        /// P1.6.
        pub p1_6: P1_6<Input<Floating>>,
        /// P1.7.
        pub p1_7: P1_7<Input<Floating>>,
        /// P1.8.
        pub p1_8: P1_8<Input<Floating>>,
        /// P1.9.
        pub p1_9: P1_9<Input<Floating>>,
        /// P1.10.
        pub p1_10: P1_10<Input<Floating>>,
        /// P1.11.
        pub p1_11: P1_11<Input<Floating>>,
        /// P1.12.
        pub p1_12: P1_12<Input<Floating>>,
        /// P1.13.
        pub p1_13: P1_13<Input<Floating>>,
        /// P1.14.
        pub p1_14: P1_14<Input<Floating>>,
        /// P1.15.
        pub p1_15: P1_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT1 {}

    impl PortExt for crate::pac::PORT1 {
        type Parts = PORT1Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT1Parts {
                p1_0: P1_0 { _mode: PhantomData },
                p1_1: P1_1 { _mode: PhantomData },
                p1_2: P1_2 { _mode: PhantomData },
                p1_3: P1_3 { _mode: PhantomData },
                p1_4: P1_4 { _mode: PhantomData },
                p1_5: P1_5 { _mode: PhantomData },
                p1_6: P1_6 { _mode: PhantomData },
                p1_7: P1_7 { _mode: PhantomData },
                p1_8: P1_8 { _mode: PhantomData },
                p1_9: P1_9 { _mode: PhantomData },
                p1_10: P1_10 { _mode: PhantomData },
                p1_11: P1_11 { _mode: PhantomData },
                p1_12: P1_12 { _mode: PhantomData },
                p1_13: P1_13 { _mode: PhantomData },
                p1_14: P1_14 { _mode: PhantomData },
                p1_15: P1_15 { _mode: PhantomData },
            };
            parts.p1_0.make_floating();
            parts.p1_1.make_floating();
            parts.p1_2.make_floating();
            parts.p1_3.make_floating();
            parts.p1_4.make_floating();
            parts.p1_5.make_floating();
            parts.p1_6.make_floating();
            parts.p1_7.make_floating();
            parts.p1_8.make_floating();
            parts.p1_9.make_floating();
            parts.p1_10.make_floating();
            parts.p1_11.make_floating();
            parts.p1_12.make_floating();
            parts.p1_13.make_floating();
            parts.p1_14.make_floating();
            parts.p1_15.make_floating();
            parts
        }
    }

    pin!(PORT1, P1_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT1, P1_0, pdr0, pd0);
    pin!(PORT1, P1_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT1, P1_1, pdr0, pd1);
    pin!(PORT1, P1_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT1, P1_2, pdr0, pd2);
    pin!(PORT1, P1_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT1, P1_3, pdr0, pd3);
    pin!(PORT1, P1_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT1, P1_4, pdr0, pd4);
    pin!(PORT1, P1_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT1, P1_5, pdr0, pd5);
    pin!(PORT1, P1_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT1, P1_6, pdr0, pd6);
    pin!(PORT1, P1_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT1, P1_7, pdr0, pd7);
    pin!(PORT1, P1_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT1, P1_8, pdr1, pd8);
    pin!(PORT1, P1_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT1, P1_9, pdr1, pd9);
    pin!(PORT1, P1_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT1, P1_10, pdr1, pd10);
    pin!(PORT1, P1_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT1, P1_11, pdr1, pd11);
    pin!(PORT1, P1_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT1, P1_12, pdr1, pd12);
    pin!(PORT1, P1_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT1, P1_13, pdr1, pd13);
    pin!(PORT1, P1_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT1, P1_14, pdr1, pd14);
    pin!(PORT1, P1_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT1, P1_15, pdr1, pd15);

    /// Pins of `PORT2`.
    pub struct PORT2Parts {
        /// P2.0.
        pub p2_0: P2_0<Input<Floating>>,
        /// P2.1.
        pub p2_1: P2_1<Input<Floating>>,
        /// P2.2.
        pub p2_2: P2_2<Input<Floating>>,
        /// P2.3.
        pub p2_3: P2_3<Input<Floating>>,
        /// P2.4.
        pub p2_4: P2_4<Input<Floating>>,
        /// P2.5.
        pub p2_5: P2_5<Input<Floating>>,
        /// P2.6.
        pub p2_6: P2_6<Input<Floating>>,
        /// P2.7.
        pub p2_7: P2_7<Input<Floating>>,
        /// P2.8.
        pub p2_8: P2_8<Input<Floating>>,
        /// P2.9.
        pub p2_9: P2_9<Input<Floating>>,
        /// P2.10.
        pub p2_10: P2_10<Input<Floating>>,
        /// P2.11.
        pub p2_11: P2_11<Input<Floating>>,
        /// P2.12.
        pub p2_12: P2_12<Input<Floating>>,
        /// P2.13.
        pub p2_13: P2_13<Input<Floating>>,
        /// P2.14.
        pub p2_14: P2_14<Input<Floating>>,
        /// P2.15.
        pub p2_15: P2_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT2 {}

    impl PortExt for crate::pac::PORT2 {
        type Parts = PORT2Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT2Parts {
                p2_0: P2_0 { _mode: PhantomData },
                p2_1: P2_1 { _mode: PhantomData },
                p2_2: P2_2 { _mode: PhantomData },
                p2_3: P2_3 { _mode: PhantomData },
                p2_4: P2_4 { _mode: PhantomData },
                p2_5: P2_5 { _mode: PhantomData },
                p2_6: P2_6 { _mode: PhantomData },
                p2_7: P2_7 { _mode: PhantomData },
                p2_8: P2_8 { _mode: PhantomData },
                p2_9: P2_9 { _mode: PhantomData },
                p2_10: P2_10 { _mode: PhantomData },
                p2_11: P2_11 { _mode: PhantomData },
                p2_12: P2_12 { _mode: PhantomData },
                p2_13: P2_13 { _mode: PhantomData },
                p2_14: P2_14 { _mode: PhantomData },
                p2_15: P2_15 { _mode: PhantomData },
            };
            parts.p2_0.make_floating();
            parts.p2_1.make_floating();
            parts.p2_2.make_floating();
            parts.p2_3.make_floating();
            parts.p2_4.make_floating();
            parts.p2_5.make_floating();
            parts.p2_6.make_floating();
            parts.p2_7.make_floating();
            parts.p2_8.make_floating();
            parts.p2_9.make_floating();
            parts.p2_10.make_floating();
            parts.p2_11.make_floating();
            parts.p2_12.make_floating();
            parts.p2_13.make_floating();
            parts.p2_14.make_floating();
            parts.p2_15.make_floating();
            parts
        }
    }

    pin!(PORT2, P2_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT2, P2_0, pdr0, pd0);
    pin!(PORT2, P2_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT2, P2_1, pdr0, pd1);
    pin!(PORT2, P2_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT2, P2_2, pdr0, pd2);
    pin!(PORT2, P2_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT2, P2_3, pdr0, pd3);
    pin!(PORT2, P2_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT2, P2_4, pdr0, pd4);
    pin!(PORT2, P2_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT2, P2_5, pdr0, pd5);
    pin!(PORT2, P2_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT2, P2_6, pdr0, pd6);
    pin!(PORT2, P2_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT2, P2_7, pdr0, pd7);
    pin!(PORT2, P2_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT2, P2_8, pdr1, pd8);
    pin!(PORT2, P2_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT2, P2_9, pdr1, pd9);
    pin!(PORT2, P2_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT2, P2_10, pdr1, pd10);
    pin!(PORT2, P2_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT2, P2_11, pdr1, pd11);
    pin!(PORT2, P2_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT2, P2_12, pdr1, pd12);
    pin!(PORT2, P2_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT2, P2_13, pdr1, pd13);
    pin!(PORT2, P2_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT2, P2_14, pdr1, pd14);
    pin!(PORT2, P2_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT2, P2_15, pdr1, pd15);

    /// Pins of `PORT3`.
    pub struct PORT3Parts {
        /// P3.0.
        pub p3_0: P3_0<Input<Floating>>,
        /// P3.1.
        pub p3_1: P3_1<Input<Floating>>,
        /// P3.2.
        pub p3_2: P3_2<Input<Floating>>,
        /// P3.3.
        pub p3_3: P3_3<Input<Floating>>,
        /// P3.4.
        pub p3_4: P3_4<Input<Floating>>,
        /// P3.5.
        pub p3_5: P3_5<Input<Floating>>,
        /// P3.6.
        pub p3_6: P3_6<Input<Floating>>,
        /// P3.7.
        pub p3_7: P3_7<Input<Floating>>,
        /// P3.8.
        pub p3_8: P3_8<Input<Floating>>,
        /// P3.9.
        pub p3_9: P3_9<Input<Floating>>,
        /// P3.10.
        pub p3_10: P3_10<Input<Floating>>,
        /// P3.11.
        pub p3_11: P3_11<Input<Floating>>,
        /// P3.12.
        pub p3_12: P3_12<Input<Floating>>,
        /// P3.13.
        pub p3_13: P3_13<Input<Floating>>,
        /// P3.14.
        pub p3_14: P3_14<Input<Floating>>,
        /// P3.15.
        pub p3_15: P3_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT3 {}

    impl PortExt for crate::pac::PORT3 {
        type Parts = PORT3Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT3Parts {
                p3_0: P3_0 { _mode: PhantomData },
                p3_1: P3_1 { _mode: PhantomData },
                p3_2: P3_2 { _mode: PhantomData },
                p3_3: P3_3 { _mode: PhantomData },
                p3_4: P3_4 { _mode: PhantomData },
                p3_5: P3_5 { _mode: PhantomData },
                p3_6: P3_6 { _mode: PhantomData },
                p3_7: P3_7 { _mode: PhantomData },
                p3_8: P3_8 { _mode: PhantomData },
                p3_9: P3_9 { _mode: PhantomData },
                p3_10: P3_10 { _mode: PhantomData },
                p3_11: P3_11 { _mode: PhantomData },
                p3_12: P3_12 { _mode: PhantomData },
                p3_13: P3_13 { _mode: PhantomData },
                p3_14: P3_14 { _mode: PhantomData },
                p3_15: P3_15 { _mode: PhantomData },
            };
            parts.p3_0.make_floating();
            parts.p3_1.make_floating();
            parts.p3_2.make_floating();
            parts.p3_3.make_floating();
            parts.p3_4.make_floating();
            parts.p3_5.make_floating();
            parts.p3_6.make_floating();
            parts.p3_7.make_floating();
            parts.p3_8.make_floating();
            parts.p3_9.make_floating();
            parts.p3_10.make_floating();
            parts.p3_11.make_floating();
            parts.p3_12.make_floating();
            parts.p3_13.make_floating();
            parts.p3_14.make_floating();
            parts.p3_15.make_floating();
            parts
        }
    }

    pin!(PORT3, P3_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT3, P3_0, pdr0, pd0);
    pin!(PORT3, P3_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT3, P3_1, pdr0, pd1);
    pin!(PORT3, P3_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT3, P3_2, pdr0, pd2);
    pin!(PORT3, P3_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT3, P3_3, pdr0, pd3);
    pin!(PORT3, P3_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT3, P3_4, pdr0, pd4);
    pin!(PORT3, P3_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT3, P3_5, pdr0, pd5);
    pin!(PORT3, P3_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT3, P3_6, pdr0, pd6);
    pin!(PORT3, P3_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT3, P3_7, pdr0, pd7);
    pin!(PORT3, P3_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT3, P3_8, pdr1, pd8);
    pin!(PORT3, P3_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT3, P3_9, pdr1, pd9);
    pin!(PORT3, P3_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT3, P3_10, pdr1, pd10);
    pin!(PORT3, P3_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT3, P3_11, pdr1, pd11);
    pin!(PORT3, P3_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT3, P3_12, pdr1, pd12);
    pin!(PORT3, P3_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT3, P3_13, pdr1, pd13);
    pin!(PORT3, P3_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT3, P3_14, pdr1, pd14);
    pin!(PORT3, P3_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT3, P3_15, pdr1, pd15);

    /// Pins of `PORT4`.
    pub struct PORT4Parts {
        /// P4.0.
        pub p4_0: P4_0<Input<Floating>>,
        /// P4.1.
        pub p4_1: P4_1<Input<Floating>>,
        /// P4.2.
        pub p4_2: P4_2<Input<Floating>>,
        /// P4.3.
        pub p4_3: P4_3<Input<Floating>>,
        /// P4.4.
        pub p4_4: P4_4<Input<Floating>>,
        /// P4.5.
        pub p4_5: P4_5<Input<Floating>>,
        /// P4.6.
        pub p4_6: P4_6<Input<Floating>>,
        /// P4.7.
        pub p4_7: P4_7<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT4 {}

    impl PortExt for crate::pac::PORT4 {
        type Parts = PORT4Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT4Parts {
                p4_0: P4_0 { _mode: PhantomData },
                p4_1: P4_1 { _mode: PhantomData },
                p4_2: P4_2 { _mode: PhantomData },
                p4_3: P4_3 { _mode: PhantomData },
                p4_4: P4_4 { _mode: PhantomData },
                p4_5: P4_5 { _mode: PhantomData },
                p4_6: P4_6 { _mode: PhantomData },
                p4_7: P4_7 { _mode: PhantomData },
            };
            parts.p4_0.make_floating();
            parts.p4_1.make_floating();
            parts.p4_2.make_floating();
            parts.p4_3.make_floating();
            parts.p4_4.make_floating();
            parts.p4_5.make_floating();
            parts.p4_6.make_floating();
            parts.p4_7.make_floating();
            parts
        }
    }

    pin!(PORT4, P4_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT4, P4_0, pdr0, pd0);
    pin!(PORT4, P4_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT4, P4_1, pdr0, pd1);
    pin!(PORT4, P4_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT4, P4_2, pdr0, pd2);
    pin!(PORT4, P4_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT4, P4_3, pdr0, pd3);
    pin!(PORT4, P4_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT4, P4_4, pdr0, pd4);
    pin!(PORT4, P4_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT4, P4_5, pdr0, pd5);
    pin!(PORT4, P4_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT4, P4_6, pdr0, pd6);
    pin!(PORT4, P4_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT4, P4_7, pdr0, pd7);

    /// Pins of `PORT5`.
    pub struct PORT5Parts {
        /// P5.0.
        pub p5_0: P5_0<Input<Floating>>,
        /// P5.1.
        pub p5_1: P5_1<Input<Floating>>,
        /// P5.2.
        pub p5_2: P5_2<Input<Floating>>,
        /// P5.3.
        pub p5_3: P5_3<Input<Floating>>,
        /// P5.4.
        pub p5_4: P5_4<Input<Floating>>,
        /// P5.5.
        pub p5_5: P5_5<Input<Floating>>,
        /// P5.6.
        pub p5_6: P5_6<Input<Floating>>,
        /// P5.7.
        pub p5_7: P5_7<Input<Floating>>,
        /// P5.8.
        pub p5_8: P5_8<Input<Floating>>,
        /// P5.9.
        pub p5_9: P5_9<Input<Floating>>,
        /// P5.10.
        pub p5_10: P5_10<Input<Floating>>,
        /// P5.11.
        pub p5_11: P5_11<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT5 {}

    impl PortExt for crate::pac::PORT5 {
        type Parts = PORT5Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT5Parts {
                p5_0: P5_0 { _mode: PhantomData },
                p5_1: P5_1 { _mode: PhantomData },
                p5_2: P5_2 { _mode: PhantomData },
                p5_3: P5_3 { _mode: PhantomData },
                p5_4: P5_4 { _mode: PhantomData },
                p5_5: P5_5 { _mode: PhantomData },
                p5_6: P5_6 { _mode: PhantomData },
                p5_7: P5_7 { _mode: PhantomData },
                p5_8: P5_8 { _mode: PhantomData },
                p5_9: P5_9 { _mode: PhantomData },
                p5_10: P5_10 { _mode: PhantomData },
                p5_11: P5_11 { _mode: PhantomData },
            };
            parts.p5_0.make_floating();
            parts.p5_1.make_floating();
            parts.p5_2.make_floating();
            parts.p5_3.make_floating();
            parts.p5_4.make_floating();
            parts.p5_5.make_floating();
            parts.p5_6.make_floating();
            parts.p5_7.make_floating();
            parts.p5_8.make_floating();
            parts.p5_9.make_floating();
            parts.p5_10.make_floating();
            parts.p5_11.make_floating();
            parts
        }
    }

    pin!(PORT5, P5_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT5, P5_0, pdr0, pd0);
    pin!(PORT5, P5_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT5, P5_1, pdr0, pd1);
    pin!(PORT5, P5_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT5, P5_2, pdr0, pd2);
    pin!(PORT5, P5_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT5, P5_3, pdr0, pd3);
    pin!(PORT5, P5_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT5, P5_4, pdr0, pd4);
    pin!(PORT5, P5_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT5, P5_5, pdr0, pd5);
    pin!(PORT5, P5_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT5, P5_6, pdr0, pd6);
    pin!(PORT5, P5_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT5, P5_7, pdr0, pd7);
    pin!(PORT5, P5_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT5, P5_8, pdr1, pd8);
    pin!(PORT5, P5_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT5, P5_9, pdr1, pd9);
    pin!(PORT5, P5_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT5, P5_10, pdr1, pd10);
    pin!(PORT5, P5_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT5, P5_11, pdr1, pd11);

    /// Pins of `PORT6`.
    pub struct PORT6Parts {
        /// P6.0.
        pub p6_0: P6_0<Input<Floating>>,
        /// P6.1.
        pub p6_1: P6_1<Input<Floating>>,
        /// P6.2.
        pub p6_2: P6_2<Input<Floating>>,
        /// P6.3.
        pub p6_3: P6_3<Input<Floating>>,
        /// P6.4.
        pub p6_4: P6_4<Input<Floating>>,
        /// P6.5.
        pub p6_5: P6_5<Input<Floating>>,
        /// P6.6.
        pub p6_6: P6_6<Input<Floating>>,
        /// P6.7.
        pub p6_7: P6_7<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT6 {}

    impl PortExt for crate::pac::PORT6 {
        type Parts = PORT6Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT6Parts {
                p6_0: P6_0 { _mode: PhantomData },
                p6_1: P6_1 { _mode: PhantomData },
                p6_2: P6_2 { _mode: PhantomData },
                p6_3: P6_3 { _mode: PhantomData },
                p6_4: P6_4 { _mode: PhantomData },
                p6_5: P6_5 { _mode: PhantomData },
                p6_6: P6_6 { _mode: PhantomData },
                p6_7: P6_7 { _mode: PhantomData },
            };
            parts.p6_0.make_floating();
            parts.p6_1.make_floating();
            parts.p6_2.make_floating();
            parts.p6_3.make_floating();
            parts.p6_4.make_floating();
            parts.p6_5.make_floating();
            parts.p6_6.make_floating();
            parts.p6_7.make_floating();
            parts
        }
    }

    pin!(PORT6, P6_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT6, P6_0, pdr0, pd0);
    pin!(PORT6, P6_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT6, P6_1, pdr0, pd1);
    pin!(PORT6, P6_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT6, P6_2, pdr0, pd2);
    pin!(PORT6, P6_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT6, P6_3, pdr0, pd3);
    pin!(PORT6, P6_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT6, P6_4, pdr0, pd4);
    pin!(PORT6, P6_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT6, P6_5, pdr0, pd5);
    pin!(PORT6, P6_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT6, P6_6, pdr0, pd6);
    pin!(PORT6, P6_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT6, P6_7, pdr0, pd7);

    /// Pins of `PORT14`.
    pub struct PORT14Parts {
        /// P14.0.
        pub p14_0: P14_0<Input<Floating>>,
        /// P14.1.
        pub p14_1: P14_1<Input<Floating>>,
        /// P14.2.
        pub p14_2: P14_2<Input<Floating>>,
        /// P14.3.
        pub p14_3: P14_3<Input<Floating>>,
        /// P14.4.
        pub p14_4: P14_4<Input<Floating>>,
        /// P14.5.
        pub p14_5: P14_5<Input<Floating>>,
        /// P14.6.
        pub p14_6: P14_6<Input<Floating>>,
        /// P14.7.
        pub p14_7: P14_7<Input<Floating>>,
        /// P14.8.
        pub p14_8: P14_8<Input<Floating>>,
        /// P14.9.
        pub p14_9: P14_9<Input<Floating>>,
        /// P14.10.
        pub p14_10: P14_10<Input<Floating>>,
        /// P14.11.
        pub p14_11: P14_11<Input<Floating>>,
        /// P14.12.
        pub p14_12: P14_12<Input<Floating>>,
        /// P14.13.
        pub p14_13: P14_13<Input<Floating>>,
        /// P14.14.
        pub p14_14: P14_14<Input<Floating>>,
        /// P14.15.
        pub p14_15: P14_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT14 {}

    impl PortExt for crate::pac::PORT14 {
        type Parts = PORT14Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT14Parts {
                p14_0: P14_0 { _mode: PhantomData },
                p14_1: P14_1 { _mode: PhantomData },
                p14_2: P14_2 { _mode: PhantomData },
                p14_3: P14_3 { _mode: PhantomData },
                p14_4: P14_4 { _mode: PhantomData },
                p14_5: P14_5 { _mode: PhantomData },
                p14_6: P14_6 { _mode: PhantomData },
                p14_7: P14_7 { _mode: PhantomData },
                p14_8: P14_8 { _mode: PhantomData },
                p14_9: P14_9 { _mode: PhantomData },
                p14_10: P14_10 { _mode: PhantomData },
                p14_11: P14_11 { _mode: PhantomData },
                p14_12: P14_12 { _mode: PhantomData },
                p14_13: P14_13 { _mode: PhantomData },
                p14_14: P14_14 { _mode: PhantomData },
                p14_15: P14_15 { _mode: PhantomData },
            };
            parts.p14_0.make_floating();
            parts.p14_1.make_floating();
            parts.p14_2.make_floating();
            parts.p14_3.make_floating();
            parts.p14_4.make_floating();
            parts.p14_5.make_floating();
            parts.p14_6.make_floating();
            parts.p14_7.make_floating();
            parts.p14_8.make_floating();
            parts.p14_9.make_floating();
            parts.p14_10.make_floating();
            parts.p14_11.make_floating();
            parts.p14_12.make_floating();
            parts.p14_13.make_floating();
            parts.p14_14.make_floating();
            parts.p14_15.make_floating();
            parts
        }
    }

    pin!(PORT14, P14_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin!(PORT14, P14_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin!(PORT14, P14_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin!(PORT14, P14_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin!(PORT14, P14_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin!(PORT14, P14_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin!(PORT14, P14_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin!(PORT14, P14_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin!(PORT14, P14_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin!(PORT14, P14_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin!(PORT14, P14_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin!(PORT14, P14_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin!(PORT14, P14_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin!(PORT14, P14_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin!(PORT14, P14_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin!(PORT14, P14_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);

    /// Pins of `PORT15`.
    pub struct PORT15Parts {
        /// P15.0.
        pub p15_0: P15_0<Input<Floating>>,
        /// P15.1.
        pub p15_1: P15_1<Input<Floating>>,
        /// P15.2.
        pub p15_2: P15_2<Input<Floating>>,
        /// P15.3.
        pub p15_3: P15_3<Input<Floating>>,
        /// P15.4.
        pub p15_4: P15_4<Input<Floating>>,
        /// P15.5.
        pub p15_5: P15_5<Input<Floating>>,
        /// P15.6.
        pub p15_6: P15_6<Input<Floating>>,
        /// P15.7.
        pub p15_7: P15_7<Input<Floating>>,
        /// P15.8.
        pub p15_8: P15_8<Input<Floating>>,
        /// P15.9.
        pub p15_9: P15_9<Input<Floating>>,
        /// P15.10.
        pub p15_10: P15_10<Input<Floating>>,
        /// P15.11.
        pub p15_11: P15_11<Input<Floating>>,
        /// P15.12.
        pub p15_12: P15_12<Input<Floating>>,
        /// P15.13.
        pub p15_13: P15_13<Input<Floating>>,
        /// P15.14.
        pub p15_14: P15_14<Input<Floating>>,
        /// P15.15.
        pub p15_15: P15_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT15 {}

    impl PortExt for crate::pac::PORT15 {
        type Parts = PORT15Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT15Parts {
                p15_0: P15_0 { _mode: PhantomData },
                p15_1: P15_1 { _mode: PhantomData },
                p15_2: P15_2 { _mode: PhantomData },
                p15_3: P15_3 { _mode: PhantomData },
                p15_4: P15_4 { _mode: PhantomData },
                p15_5: P15_5 { _mode: PhantomData },
                p15_6: P15_6 { _mode: PhantomData },
                p15_7: P15_7 { _mode: PhantomData },
                p15_8: P15_8 { _mode: PhantomData },
                p15_9: P15_9 { _mode: PhantomData },
                p15_10: P15_10 { _mode: PhantomData },
                p15_11: P15_11 { _mode: PhantomData },
                p15_12: P15_12 { _mode: PhantomData },
                p15_13: P15_13 { _mode: PhantomData },
                p15_14: P15_14 { _mode: PhantomData },
                p15_15: P15_15 { _mode: PhantomData },
            };
            parts.p15_0.make_floating();
            parts.p15_1.make_floating();
            parts.p15_2.make_floating();
            parts.p15_3.make_floating();
            parts.p15_4.make_floating();
            parts.p15_5.make_floating();
            parts.p15_6.make_floating();
            parts.p15_7.make_floating();
            parts.p15_8.make_floating();
            parts.p15_9.make_floating();
            parts.p15_10.make_floating();
            parts.p15_11.make_floating();
            parts.p15_12.make_floating();
            parts.p15_13.make_floating();
            parts.p15_14.make_floating();
            parts.p15_15.make_floating();
            parts
        }
    }

    pin!(PORT15, P15_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin!(PORT15, P15_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin!(PORT15, P15_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin!(PORT15, P15_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin!(PORT15, P15_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin!(PORT15, P15_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin!(PORT15, P15_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin!(PORT15, P15_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin!(PORT15, P15_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin!(PORT15, P15_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin!(PORT15, P15_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin!(PORT15, P15_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin!(PORT15, P15_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin!(PORT15, P15_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin!(PORT15, P15_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin!(PORT15, P15_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
}

#[cfg(any(feature = "xmc4500"))]
pub use xmc4500::*;

#[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
mod large {
    use super::*;

    /// Pins of `PORT0`.
    pub struct PORT0Parts {
        /// P0.0.
        pub p0_0: P0_0<Input<Floating>>,
        /// P0.1.
        pub p0_1: P0_1<Input<Floating>>,
        /// P0.2.
        pub p0_2: P0_2<Input<Floating>>,
        /// P0.3.
        pub p0_3: P0_3<Input<Floating>>,
        /// P0.4.
        pub p0_4: P0_4<Input<Floating>>,
        /// P0.5.
        pub p0_5: P0_5<Input<Floating>>,
        /// P0.6.
        pub p0_6: P0_6<Input<Floating>>,
        /// P0.7.
        pub p0_7: P0_7<Input<Floating>>,
        /// P0.8.
        pub p0_8: P0_8<Input<Floating>>,
        /// P0.9.
        pub p0_9: P0_9<Input<Floating>>,
        /// P0.10.
        pub p0_10: P0_10<Input<Floating>>,
        /// P0.11.
        pub p0_11: P0_11<Input<Floating>>,
        /// P0.12.
        pub p0_12: P0_12<Input<Floating>>,
        /// P0.13.
        pub p0_13: P0_13<Input<Floating>>,
        /// P0.14.
        pub p0_14: P0_14<Input<Floating>>,
        /// P0.15.
        pub p0_15: P0_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT0 {}

    impl PortExt for crate::pac::PORT0 {
        type Parts = PORT0Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT0Parts {
                p0_0: P0_0 { _mode: PhantomData },
                p0_1: P0_1 { _mode: PhantomData },
                p0_2: P0_2 { _mode: PhantomData },
                p0_3: P0_3 { _mode: PhantomData },
                p0_4: P0_4 { _mode: PhantomData },
                p0_5: P0_5 { _mode: PhantomData },
                p0_6: P0_6 { _mode: PhantomData },
                p0_7: P0_7 { _mode: PhantomData },
                p0_8: P0_8 { _mode: PhantomData },
                p0_9: P0_9 { _mode: PhantomData },
                p0_10: P0_10 { _mode: PhantomData },
                p0_11: P0_11 { _mode: PhantomData },
                p0_12: P0_12 { _mode: PhantomData },
                p0_13: P0_13 { _mode: PhantomData },
                p0_14: P0_14 { _mode: PhantomData },
                p0_15: P0_15 { _mode: PhantomData },
            };
            parts.p0_0.make_floating();
            parts.p0_1.make_floating();
            parts.p0_2.make_floating();
            parts.p0_3.make_floating();
            parts.p0_4.make_floating();
            parts.p0_5.make_floating();
            parts.p0_6.make_floating();
            parts.p0_7.make_floating();
            parts.p0_8.make_floating();
            parts.p0_9.make_floating();
            parts.p0_10.make_floating();
            parts.p0_11.make_floating();
            parts.p0_12.make_floating();
            parts.p0_13.make_floating();
            parts.p0_14.make_floating();
            parts.p0_15.make_floating();
            parts
        }
    }

    pin!(PORT0, P0_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT0, P0_0, pdr0, pd0);
    pin!(PORT0, P0_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT0, P0_1, pdr0, pd1);
    pin!(PORT0, P0_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT0, P0_2, pdr0, pd2);
    pin!(PORT0, P0_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT0, P0_3, pdr0, pd3);
    pin!(PORT0, P0_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT0, P0_4, pdr0, pd4);
    pin!(PORT0, P0_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT0, P0_5, pdr0, pd5);
    pin!(PORT0, P0_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT0, P0_6, pdr0, pd6);
    pin!(PORT0, P0_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT0, P0_7, pdr0, pd7);
    pin!(PORT0, P0_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT0, P0_8, pdr1, pd8);
    pin!(PORT0, P0_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT0, P0_9, pdr1, pd9);
    pin!(PORT0, P0_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT0, P0_10, pdr1, pd10);
    pin!(PORT0, P0_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT0, P0_11, pdr1, pd11);
    pin!(PORT0, P0_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT0, P0_12, pdr1, pd12);
    pin!(PORT0, P0_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT0, P0_13, pdr1, pd13);
    pin!(PORT0, P0_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT0, P0_14, pdr1, pd14);
    pin!(PORT0, P0_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT0, P0_15, pdr1, pd15);

    /// Pins of `PORT1`.
    pub struct PORT1Parts {
        /// P1.0.
        pub p1_0: P1_0<Input<Floating>>,
        /// P1.1.
        pub p1_1: P1_1<Input<Floating>>,
        /// P1.2.
        pub p1_2: P1_2<Input<Floating>>,
        /// P1.3.
        pub p1_3: P1_3<Input<Floating>>,
        /// P1.4.
        pub p1_4: P1_4<Input<Floating>>,
        /// P1.5.
        pub p1_5: P1_5<Input<Floating>>,
        /// P1.6.
        pub p1_6: P1_6<Input<Floating>>,
        /// P1.7.
        pub p1_7: P1_7<Input<Floating>>,
        /// P1.8.
        pub p1_8: P1_8<Input<Floating>>,
        /// P1.9.
        pub p1_9: P1_9<Input<Floating>>,
        /// P1.10.
        pub p1_10: P1_10<Input<Floating>>,
        /// P1.11.
        pub p1_11: P1_11<Input<Floating>>,
        /// P1.12.
        pub p1_12: P1_12<Input<Floating>>,
        /// P1.13.
        pub p1_13: P1_13<Input<Floating>>,
        /// P1.14.
        pub p1_14: P1_14<Input<Floating>>,
        /// P1.15.
        pub p1_15: P1_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT1 {}

    impl PortExt for crate::pac::PORT1 {
        type Parts = PORT1Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT1Parts {
                p1_0: P1_0 { _mode: PhantomData },
                p1_1: P1_1 { _mode: PhantomData },
                p1_2: P1_2 { _mode: PhantomData },
                p1_3: P1_3 { _mode: PhantomData },
                p1_4: P1_4 { _mode: PhantomData },
                p1_5: P1_5 { _mode: PhantomData },
                p1_6: P1_6 { _mode: PhantomData },
                p1_7: P1_7 { _mode: PhantomData },
                p1_8: P1_8 { _mode: PhantomData },
                p1_9: P1_9 { _mode: PhantomData },
                p1_10: P1_10 { _mode: PhantomData },
                p1_11: P1_11 { _mode: PhantomData },
                p1_12: P1_12 { _mode: PhantomData },
                p1_13: P1_13 { _mode: PhantomData },
                p1_14: P1_14 { _mode: PhantomData },
                p1_15: P1_15 { _mode: PhantomData },
            };
            parts.p1_0.make_floating();
            parts.p1_1.make_floating();
            parts.p1_2.make_floating();
            parts.p1_3.make_floating();
            parts.p1_4.make_floating();
            parts.p1_5.make_floating();
            parts.p1_6.make_floating();
            parts.p1_7.make_floating();
            parts.p1_8.make_floating();
            parts.p1_9.make_floating();
            parts.p1_10.make_floating();
            parts.p1_11.make_floating();
            parts.p1_12.make_floating();
            parts.p1_13.make_floating();
            parts.p1_14.make_floating();
            parts.p1_15.make_floating();
            parts
        }
    }

    pin!(PORT1, P1_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT1, P1_0, pdr0, pd0);
    pin!(PORT1, P1_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT1, P1_1, pdr0, pd1);
    pin!(PORT1, P1_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT1, P1_2, pdr0, pd2);
    pin!(PORT1, P1_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT1, P1_3, pdr0, pd3);
    pin!(PORT1, P1_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT1, P1_4, pdr0, pd4);
    pin!(PORT1, P1_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT1, P1_5, pdr0, pd5);
    pin!(PORT1, P1_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT1, P1_6, pdr0, pd6);
    pin!(PORT1, P1_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT1, P1_7, pdr0, pd7);
    pin!(PORT1, P1_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT1, P1_8, pdr1, pd8);
    pin!(PORT1, P1_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT1, P1_9, pdr1, pd9);
    pin!(PORT1, P1_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT1, P1_10, pdr1, pd10);
    pin!(PORT1, P1_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT1, P1_11, pdr1, pd11);
    pin!(PORT1, P1_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT1, P1_12, pdr1, pd12);
    pin!(PORT1, P1_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT1, P1_13, pdr1, pd13);
    pin!(PORT1, P1_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT1, P1_14, pdr1, pd14);
    pin!(PORT1, P1_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT1, P1_15, pdr1, pd15);

    /// Pins of `PORT2`.
    pub struct PORT2Parts {
        /// P2.0.
        pub p2_0: P2_0<Input<Floating>>,
        /// P2.1.
        pub p2_1: P2_1<Input<Floating>>,
        /// P2.2.
        pub p2_2: P2_2<Input<Floating>>,
        /// P2.3.
        pub p2_3: P2_3<Input<Floating>>,
        /// P2.4.
        pub p2_4: P2_4<Input<Floating>>,
        /// P2.5.
        pub p2_5: P2_5<Input<Floating>>,
        /// P2.6.
        pub p2_6: P2_6<Input<Floating>>,
        /// P2.7.
        pub p2_7: P2_7<Input<Floating>>,
        /// P2.8.
        pub p2_8: P2_8<Input<Floating>>,
        /// P2.9.
        pub p2_9: P2_9<Input<Floating>>,
        /// P2.10.
        pub p2_10: P2_10<Input<Floating>>,
        /// P2.11.
        pub p2_11: P2_11<Input<Floating>>,
        /// P2.12.
        pub p2_12: P2_12<Input<Floating>>,
        /// P2.13.
        pub p2_13: P2_13<Input<Floating>>,
        /// P2.14.
        pub p2_14: P2_14<Input<Floating>>,
        /// P2.15.
        pub p2_15: P2_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT2 {}

    impl PortExt for crate::pac::PORT2 {
        type Parts = PORT2Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT2Parts {
                p2_0: P2_0 { _mode: PhantomData },
                p2_1: P2_1 { _mode: PhantomData },
                p2_2: P2_2 { _mode: PhantomData },
                p2_3: P2_3 { _mode: PhantomData },
                p2_4: P2_4 { _mode: PhantomData },
                p2_5: P2_5 { _mode: PhantomData },
                p2_6: P2_6 { _mode: PhantomData },
                p2_7: P2_7 { _mode: PhantomData },
                p2_8: P2_8 { _mode: PhantomData },
                p2_9: P2_9 { _mode: PhantomData },
                p2_10: P2_10 { _mode: PhantomData },
                p2_11: P2_11 { _mode: PhantomData },
                p2_12: P2_12 { _mode: PhantomData },
                p2_13: P2_13 { _mode: PhantomData },
                p2_14: P2_14 { _mode: PhantomData },
                p2_15: P2_15 { _mode: PhantomData },
            };
            parts.p2_0.make_floating();
            parts.p2_1.make_floating();
            parts.p2_2.make_floating();
            parts.p2_3.make_floating();
            parts.p2_4.make_floating();
            parts.p2_5.make_floating();
            parts.p2_6.make_floating();
            parts.p2_7.make_floating();
            parts.p2_8.make_floating();
            parts.p2_9.make_floating();
            parts.p2_10.make_floating();
            parts.p2_11.make_floating();
            parts.p2_12.make_floating();
            parts.p2_13.make_floating();
            parts.p2_14.make_floating();
            parts.p2_15.make_floating();
            parts
        }
    }

    pin!(PORT2, P2_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT2, P2_0, pdr0, pd0);
    pin!(PORT2, P2_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT2, P2_1, pdr0, pd1);
    pin!(PORT2, P2_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT2, P2_2, pdr0, pd2);
    pin!(PORT2, P2_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT2, P2_3, pdr0, pd3);
    pin!(PORT2, P2_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT2, P2_4, pdr0, pd4);
    pin!(PORT2, P2_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT2, P2_5, pdr0, pd5);
    pin!(PORT2, P2_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT2, P2_6, pdr0, pd6);
    pin!(PORT2, P2_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT2, P2_7, pdr0, pd7);
    pin!(PORT2, P2_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT2, P2_8, pdr1, pd8);
    pin!(PORT2, P2_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT2, P2_9, pdr1, pd9);
    pin!(PORT2, P2_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT2, P2_10, pdr1, pd10);
    pin!(PORT2, P2_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT2, P2_11, pdr1, pd11);
    pin!(PORT2, P2_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT2, P2_12, pdr1, pd12);
    pin!(PORT2, P2_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT2, P2_13, pdr1, pd13);
    pin!(PORT2, P2_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT2, P2_14, pdr1, pd14);
    pin!(PORT2, P2_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT2, P2_15, pdr1, pd15);

    /// Pins of `PORT3`.
    pub struct PORT3Parts {
        /// P3.0.
        pub p3_0: P3_0<Input<Floating>>,
        /// P3.1.
        pub p3_1: P3_1<Input<Floating>>,
        /// P3.2.
        pub p3_2: P3_2<Input<Floating>>,
        /// P3.3.
        pub p3_3: P3_3<Input<Floating>>,
        /// P3.4.
        pub p3_4: P3_4<Input<Floating>>,
        /// P3.5.
        pub p3_5: P3_5<Input<Floating>>,
        /// P3.6.
        pub p3_6: P3_6<Input<Floating>>,
        /// P3.7.
        pub p3_7: P3_7<Input<Floating>>,
        /// P3.8.
        pub p3_8: P3_8<Input<Floating>>,
        /// P3.9.
        pub p3_9: P3_9<Input<Floating>>,
        /// P3.10.
        pub p3_10: P3_10<Input<Floating>>,
        /// P3.11.
        pub p3_11: P3_11<Input<Floating>>,
        /// P3.12.
        pub p3_12: P3_12<Input<Floating>>,
        /// P3.13.
        pub p3_13: P3_13<Input<Floating>>,
        /// P3.14.
        pub p3_14: P3_14<Input<Floating>>,
        /// P3.15.
        pub p3_15: P3_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT3 {}

    impl PortExt for crate::pac::PORT3 {
        type Parts = PORT3Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT3Parts {
                p3_0: P3_0 { _mode: PhantomData },
                p3_1: P3_1 { _mode: PhantomData },
                p3_2: P3_2 { _mode: PhantomData },
                p3_3: P3_3 { _mode: PhantomData },
                p3_4: P3_4 { _mode: PhantomData },
                p3_5: P3_5 { _mode: PhantomData },
                p3_6: P3_6 { _mode: PhantomData },
                p3_7: P3_7 { _mode: PhantomData },
                p3_8: P3_8 { _mode: PhantomData },
                p3_9: P3_9 { _mode: PhantomData },
                p3_10: P3_10 { _mode: PhantomData },
                p3_11: P3_11 { _mode: PhantomData },
                p3_12: P3_12 { _mode: PhantomData },
                p3_13: P3_13 { _mode: PhantomData },
                p3_14: P3_14 { _mode: PhantomData },
                p3_15: P3_15 { _mode: PhantomData },
            };
            parts.p3_0.make_floating();
            parts.p3_1.make_floating();
            parts.p3_2.make_floating();
            parts.p3_3.make_floating();
            parts.p3_4.make_floating();
            parts.p3_5.make_floating();
            parts.p3_6.make_floating();
            parts.p3_7.make_floating();
            parts.p3_8.make_floating();
            parts.p3_9.make_floating();
            parts.p3_10.make_floating();
            parts.p3_11.make_floating();
            parts.p3_12.make_floating();
            parts.p3_13.make_floating();
            parts.p3_14.make_floating();
            parts.p3_15.make_floating();
            parts
        }
    }

    pin!(PORT3, P3_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT3, P3_0, pdr0, pd0);
    pin!(PORT3, P3_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT3, P3_1, pdr0, pd1);
    pin!(PORT3, P3_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT3, P3_2, pdr0, pd2);
    pin!(PORT3, P3_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT3, P3_3, pdr0, pd3);
    pin!(PORT3, P3_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT3, P3_4, pdr0, pd4);
    pin!(PORT3, P3_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT3, P3_5, pdr0, pd5);
    pin!(PORT3, P3_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT3, P3_6, pdr0, pd6);
    pin!(PORT3, P3_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT3, P3_7, pdr0, pd7);
    pin!(PORT3, P3_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT3, P3_8, pdr1, pd8);
    pin!(PORT3, P3_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT3, P3_9, pdr1, pd9);
    pin!(PORT3, P3_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT3, P3_10, pdr1, pd10);
    pin!(PORT3, P3_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT3, P3_11, pdr1, pd11);
    pin!(PORT3, P3_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin_drive!(PORT3, P3_12, pdr1, pd12);
    pin!(PORT3, P3_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin_drive!(PORT3, P3_13, pdr1, pd13);
    pin!(PORT3, P3_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin_drive!(PORT3, P3_14, pdr1, pd14);
    pin!(PORT3, P3_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
    pin_drive!(PORT3, P3_15, pdr1, pd15);

    /// Pins of `PORT4`.
    pub struct PORT4Parts {
        /// P4.0.
        pub p4_0: P4_0<Input<Floating>>,
        /// P4.1.
        pub p4_1: P4_1<Input<Floating>>,
        /// P4.2.
        pub p4_2: P4_2<Input<Floating>>,
        /// P4.3.
        pub p4_3: P4_3<Input<Floating>>,
        /// P4.4.
        pub p4_4: P4_4<Input<Floating>>,
        /// P4.5.
        pub p4_5: P4_5<Input<Floating>>,
        /// P4.6.
        pub p4_6: P4_6<Input<Floating>>,
        /// P4.7.
        pub p4_7: P4_7<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT4 {}

    impl PortExt for crate::pac::PORT4 {
        type Parts = PORT4Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT4Parts {
                p4_0: P4_0 { _mode: PhantomData },
                p4_1: P4_1 { _mode: PhantomData },
                p4_2: P4_2 { _mode: PhantomData },
                p4_3: P4_3 { _mode: PhantomData },
                p4_4: P4_4 { _mode: PhantomData },
                p4_5: P4_5 { _mode: PhantomData },
                p4_6: P4_6 { _mode: PhantomData },
                p4_7: P4_7 { _mode: PhantomData },
            };
            parts.p4_0.make_floating();
            parts.p4_1.make_floating();
            parts.p4_2.make_floating();
            parts.p4_3.make_floating();
            parts.p4_4.make_floating();
            parts.p4_5.make_floating();
            parts.p4_6.make_floating();
            parts.p4_7.make_floating();
            parts
        }
    }

    pin!(PORT4, P4_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT4, P4_0, pdr0, pd0);
    pin!(PORT4, P4_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT4, P4_1, pdr0, pd1);
    pin!(PORT4, P4_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT4, P4_2, pdr0, pd2);
    pin!(PORT4, P4_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT4, P4_3, pdr0, pd3);
    pin!(PORT4, P4_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT4, P4_4, pdr0, pd4);
    pin!(PORT4, P4_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT4, P4_5, pdr0, pd5);
    pin!(PORT4, P4_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT4, P4_6, pdr0, pd6);
    pin!(PORT4, P4_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT4, P4_7, pdr0, pd7);

    /// Pins of `PORT5`.
    pub struct PORT5Parts {
        /// P5.0.
        pub p5_0: P5_0<Input<Floating>>,
        /// P5.1.
        pub p5_1: P5_1<Input<Floating>>,
        /// P5.2.
        pub p5_2: P5_2<Input<Floating>>,
        /// P5.3.
        pub p5_3: P5_3<Input<Floating>>,
        /// P5.4.
        pub p5_4: P5_4<Input<Floating>>,
        /// P5.5.
        pub p5_5: P5_5<Input<Floating>>,
        /// P5.6.
        pub p5_6: P5_6<Input<Floating>>,
        /// P5.7.
        pub p5_7: P5_7<Input<Floating>>,
        /// P5.8.
        pub p5_8: P5_8<Input<Floating>>,
        /// P5.9.
        pub p5_9: P5_9<Input<Floating>>,
        /// P5.10.
        pub p5_10: P5_10<Input<Floating>>,
        /// P5.11.
        pub p5_11: P5_11<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT5 {}

    impl PortExt for crate::pac::PORT5 {
        type Parts = PORT5Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT5Parts {
                p5_0: P5_0 { _mode: PhantomData },
                p5_1: P5_1 { _mode: PhantomData },
                p5_2: P5_2 { _mode: PhantomData },
                p5_3: P5_3 { _mode: PhantomData },
                p5_4: P5_4 { _mode: PhantomData },
                p5_5: P5_5 { _mode: PhantomData },
                p5_6: P5_6 { _mode: PhantomData },
                p5_7: P5_7 { _mode: PhantomData },
                p5_8: P5_8 { _mode: PhantomData },
                p5_9: P5_9 { _mode: PhantomData },
                p5_10: P5_10 { _mode: PhantomData },
                p5_11: P5_11 { _mode: PhantomData },
            };
            parts.p5_0.make_floating();
            parts.p5_1.make_floating();
            parts.p5_2.make_floating();
            parts.p5_3.make_floating();
            parts.p5_4.make_floating();
            parts.p5_5.make_floating();
            parts.p5_6.make_floating();
            parts.p5_7.make_floating();
            parts.p5_8.make_floating();
            parts.p5_9.make_floating();
            parts.p5_10.make_floating();
            parts.p5_11.make_floating();
            parts
        }
    }

    pin!(PORT5, P5_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT5, P5_0, pdr0, pd0);
    pin!(PORT5, P5_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT5, P5_1, pdr0, pd1);
    pin!(PORT5, P5_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT5, P5_2, pdr0, pd2);
    pin!(PORT5, P5_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT5, P5_3, pdr0, pd3);
    pin!(PORT5, P5_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT5, P5_4, pdr0, pd4);
    pin!(PORT5, P5_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT5, P5_5, pdr0, pd5);
    pin!(PORT5, P5_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT5, P5_6, pdr0, pd6);
    pin!(PORT5, P5_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT5, P5_7, pdr0, pd7);
    pin!(PORT5, P5_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT5, P5_8, pdr1, pd8);
    pin!(PORT5, P5_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT5, P5_9, pdr1, pd9);
    pin!(PORT5, P5_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT5, P5_10, pdr1, pd10);
    pin!(PORT5, P5_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT5, P5_11, pdr1, pd11);

    /// Pins of `PORT6`.
    pub struct PORT6Parts {
        /// P6.0.
        pub p6_0: P6_0<Input<Floating>>,
        /// P6.1.
        pub p6_1: P6_1<Input<Floating>>,
        /// P6.2.
        pub p6_2: P6_2<Input<Floating>>,
        /// P6.3.
        pub p6_3: P6_3<Input<Floating>>,
        /// P6.4.
        pub p6_4: P6_4<Input<Floating>>,
        /// P6.5.
        pub p6_5: P6_5<Input<Floating>>,
        /// P6.6.
        pub p6_6: P6_6<Input<Floating>>,
        /// P6.7.
        pub p6_7: P6_7<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT6 {}

    impl PortExt for crate::pac::PORT6 {
        type Parts = PORT6Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT6Parts {
                p6_0: P6_0 { _mode: PhantomData },
                p6_1: P6_1 { _mode: PhantomData },
                p6_2: P6_2 { _mode: PhantomData },
                p6_3: P6_3 { _mode: PhantomData },
                p6_4: P6_4 { _mode: PhantomData },
                p6_5: P6_5 { _mode: PhantomData },
                p6_6: P6_6 { _mode: PhantomData },
                p6_7: P6_7 { _mode: PhantomData },
            };
            parts.p6_0.make_floating();
            parts.p6_1.make_floating();
            parts.p6_2.make_floating();
            parts.p6_3.make_floating();
            parts.p6_4.make_floating();
            parts.p6_5.make_floating();
            parts.p6_6.make_floating();
            parts.p6_7.make_floating();
            parts
        }
    }

    pin!(PORT6, P6_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT6, P6_0, pdr0, pd0);
    pin!(PORT6, P6_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT6, P6_1, pdr0, pd1);
    pin!(PORT6, P6_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT6, P6_2, pdr0, pd2);
    pin!(PORT6, P6_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT6, P6_3, pdr0, pd3);
    pin!(PORT6, P6_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT6, P6_4, pdr0, pd4);
    pin!(PORT6, P6_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT6, P6_5, pdr0, pd5);
    pin!(PORT6, P6_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT6, P6_6, pdr0, pd6);
    pin!(PORT6, P6_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT6, P6_7, pdr0, pd7);

    /// Pins of `PORT7`.
    pub struct PORT7Parts {
        /// P7.0.
        pub p7_0: P7_0<Input<Floating>>,
        /// P7.1.
        pub p7_1: P7_1<Input<Floating>>,
        /// P7.2.
        pub p7_2: P7_2<Input<Floating>>,
        /// P7.3.
        pub p7_3: P7_3<Input<Floating>>,
        /// P7.4.
        pub p7_4: P7_4<Input<Floating>>,
        /// P7.5.
        pub p7_5: P7_5<Input<Floating>>,
        /// P7.6.
        pub p7_6: P7_6<Input<Floating>>,
        /// P7.7.
        pub p7_7: P7_7<Input<Floating>>,
        /// P7.8.
        pub p7_8: P7_8<Input<Floating>>,
        /// P7.9.
        pub p7_9: P7_9<Input<Floating>>,
        /// P7.10.
        pub p7_10: P7_10<Input<Floating>>,
        /// P7.11.
        pub p7_11: P7_11<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT7 {}

    impl PortExt for crate::pac::PORT7 {
        type Parts = PORT7Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT7Parts {
                p7_0: P7_0 { _mode: PhantomData },
                p7_1: P7_1 { _mode: PhantomData },
                p7_2: P7_2 { _mode: PhantomData },
                p7_3: P7_3 { _mode: PhantomData },
                p7_4: P7_4 { _mode: PhantomData },
                p7_5: P7_5 { _mode: PhantomData },
                p7_6: P7_6 { _mode: PhantomData },
                p7_7: P7_7 { _mode: PhantomData },
                p7_8: P7_8 { _mode: PhantomData },
                p7_9: P7_9 { _mode: PhantomData },
                p7_10: P7_10 { _mode: PhantomData },
                p7_11: P7_11 { _mode: PhantomData },
            };
            parts.p7_0.make_floating();
            parts.p7_1.make_floating();
            parts.p7_2.make_floating();
            parts.p7_3.make_floating();
            parts.p7_4.make_floating();
            parts.p7_5.make_floating();
            parts.p7_6.make_floating();
            parts.p7_7.make_floating();
            parts.p7_8.make_floating();
            parts.p7_9.make_floating();
            parts.p7_10.make_floating();
            parts.p7_11.make_floating();
            parts
        }
    }

    pin!(PORT7, P7_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT7, P7_0, pdr0, pd0);
    pin!(PORT7, P7_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT7, P7_1, pdr0, pd1);
    pin!(PORT7, P7_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT7, P7_2, pdr0, pd2);
    pin!(PORT7, P7_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT7, P7_3, pdr0, pd3);
    pin!(PORT7, P7_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT7, P7_4, pdr0, pd4);
    pin!(PORT7, P7_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT7, P7_5, pdr0, pd5);
    pin!(PORT7, P7_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT7, P7_6, pdr0, pd6);
    pin!(PORT7, P7_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT7, P7_7, pdr0, pd7);
    pin!(PORT7, P7_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT7, P7_8, pdr1, pd8);
    pin!(PORT7, P7_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT7, P7_9, pdr1, pd9);
    pin!(PORT7, P7_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT7, P7_10, pdr1, pd10);
    pin!(PORT7, P7_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT7, P7_11, pdr1, pd11);

    /// Pins of `PORT8`.
    pub struct PORT8Parts {
        /// P8.0.
        pub p8_0: P8_0<Input<Floating>>,
        /// P8.1.
        pub p8_1: P8_1<Input<Floating>>,
        /// P8.2.
        pub p8_2: P8_2<Input<Floating>>,
        /// P8.3.
        pub p8_3: P8_3<Input<Floating>>,
        /// P8.4.
        pub p8_4: P8_4<Input<Floating>>,
        /// P8.5.
        pub p8_5: P8_5<Input<Floating>>,
        /// P8.6.
        pub p8_6: P8_6<Input<Floating>>,
        /// P8.7.
        pub p8_7: P8_7<Input<Floating>>,
        /// P8.8.
        pub p8_8: P8_8<Input<Floating>>,
        /// P8.9.
        pub p8_9: P8_9<Input<Floating>>,
        /// P8.10.
        pub p8_10: P8_10<Input<Floating>>,
        /// P8.11.
        pub p8_11: P8_11<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT8 {}

    impl PortExt for crate::pac::PORT8 {
        type Parts = PORT8Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT8Parts {
                p8_0: P8_0 { _mode: PhantomData },
                p8_1: P8_1 { _mode: PhantomData },
                p8_2: P8_2 { _mode: PhantomData },
                p8_3: P8_3 { _mode: PhantomData },
                p8_4: P8_4 { _mode: PhantomData },
                p8_5: P8_5 { _mode: PhantomData },
                p8_6: P8_6 { _mode: PhantomData },
                p8_7: P8_7 { _mode: PhantomData },
                p8_8: P8_8 { _mode: PhantomData },
                p8_9: P8_9 { _mode: PhantomData },
                p8_10: P8_10 { _mode: PhantomData },
                p8_11: P8_11 { _mode: PhantomData },
            };
            parts.p8_0.make_floating();
            parts.p8_1.make_floating();
            parts.p8_2.make_floating();
            parts.p8_3.make_floating();
            parts.p8_4.make_floating();
            parts.p8_5.make_floating();
            parts.p8_6.make_floating();
            parts.p8_7.make_floating();
            parts.p8_8.make_floating();
            parts.p8_9.make_floating();
            parts.p8_10.make_floating();
            parts.p8_11.make_floating();
            parts
        }
    }

    pin!(PORT8, P8_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT8, P8_0, pdr0, pd0);
    pin!(PORT8, P8_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT8, P8_1, pdr0, pd1);
    pin!(PORT8, P8_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT8, P8_2, pdr0, pd2);
    pin!(PORT8, P8_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT8, P8_3, pdr0, pd3);
    pin!(PORT8, P8_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT8, P8_4, pdr0, pd4);
    pin!(PORT8, P8_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT8, P8_5, pdr0, pd5);
    pin!(PORT8, P8_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT8, P8_6, pdr0, pd6);
    pin!(PORT8, P8_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT8, P8_7, pdr0, pd7);
    pin!(PORT8, P8_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT8, P8_8, pdr1, pd8);
    pin!(PORT8, P8_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT8, P8_9, pdr1, pd9);
    pin!(PORT8, P8_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT8, P8_10, pdr1, pd10);
    pin!(PORT8, P8_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT8, P8_11, pdr1, pd11);

    /// Pins of `PORT9`.
    pub struct PORT9Parts {
        /// P9.0.
        pub p9_0: P9_0<Input<Floating>>,
        /// P9.1.
        pub p9_1: P9_1<Input<Floating>>,
        /// P9.2.
        pub p9_2: P9_2<Input<Floating>>,
        /// P9.3.
        pub p9_3: P9_3<Input<Floating>>,
        /// P9.4.
        pub p9_4: P9_4<Input<Floating>>,
        /// P9.5.
        pub p9_5: P9_5<Input<Floating>>,
        /// P9.6.
        pub p9_6: P9_6<Input<Floating>>,
        /// P9.7.
        pub p9_7: P9_7<Input<Floating>>,
        /// P9.8.
        pub p9_8: P9_8<Input<Floating>>,
        /// P9.9.
        pub p9_9: P9_9<Input<Floating>>,
        /// P9.10.
        pub p9_10: P9_10<Input<Floating>>,
        /// P9.11.
        pub p9_11: P9_11<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT9 {}

    impl PortExt for crate::pac::PORT9 {
        type Parts = PORT9Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT9Parts {
                p9_0: P9_0 { _mode: PhantomData },
                p9_1: P9_1 { _mode: PhantomData },
                p9_2: P9_2 { _mode: PhantomData },
                p9_3: P9_3 { _mode: PhantomData },
                p9_4: P9_4 { _mode: PhantomData },
                p9_5: P9_5 { _mode: PhantomData },
                p9_6: P9_6 { _mode: PhantomData },
                p9_7: P9_7 { _mode: PhantomData },
                p9_8: P9_8 { _mode: PhantomData },
                p9_9: P9_9 { _mode: PhantomData },
                p9_10: P9_10 { _mode: PhantomData },
                p9_11: P9_11 { _mode: PhantomData },
            };
            parts.p9_0.make_floating();
            parts.p9_1.make_floating();
            parts.p9_2.make_floating();
            parts.p9_3.make_floating();
            parts.p9_4.make_floating();
            parts.p9_5.make_floating();
            parts.p9_6.make_floating();
            parts.p9_7.make_floating();
            parts.p9_8.make_floating();
            parts.p9_9.make_floating();
            parts.p9_10.make_floating();
            parts.p9_11.make_floating();
            parts
        }
    }

    pin!(PORT9, P9_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin_drive!(PORT9, P9_0, pdr0, pd0);
    pin!(PORT9, P9_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin_drive!(PORT9, P9_1, pdr0, pd1);
    pin!(PORT9, P9_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin_drive!(PORT9, P9_2, pdr0, pd2);
    pin!(PORT9, P9_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin_drive!(PORT9, P9_3, pdr0, pd3);
    pin!(PORT9, P9_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin_drive!(PORT9, P9_4, pdr0, pd4);
    pin!(PORT9, P9_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin_drive!(PORT9, P9_5, pdr0, pd5);
    pin!(PORT9, P9_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin_drive!(PORT9, P9_6, pdr0, pd6);
    pin!(PORT9, P9_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin_drive!(PORT9, P9_7, pdr0, pd7);
    pin!(PORT9, P9_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin_drive!(PORT9, P9_8, pdr1, pd8);
    pin!(PORT9, P9_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin_drive!(PORT9, P9_9, pdr1, pd9);
    pin!(PORT9, P9_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin_drive!(PORT9, P9_10, pdr1, pd10);
    pin!(PORT9, P9_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin_drive!(PORT9, P9_11, pdr1, pd11);

    /// Pins of `PORT14`.
    pub struct PORT14Parts {
        /// P14.0.
        pub p14_0: P14_0<Input<Floating>>,
        /// P14.1.
        pub p14_1: P14_1<Input<Floating>>,
        /// P14.2.
        pub p14_2: P14_2<Input<Floating>>,
        /// P14.3.
        pub p14_3: P14_3<Input<Floating>>,
        /// P14.4.
        pub p14_4: P14_4<Input<Floating>>,
        /// P14.5.
        pub p14_5: P14_5<Input<Floating>>,
        /// P14.6.
        pub p14_6: P14_6<Input<Floating>>,
        /// P14.7.
        pub p14_7: P14_7<Input<Floating>>,
        /// P14.8.
        pub p14_8: P14_8<Input<Floating>>,
        /// P14.9.
        pub p14_9: P14_9<Input<Floating>>,
        /// P14.10.
        pub p14_10: P14_10<Input<Floating>>,
        /// P14.11.
        pub p14_11: P14_11<Input<Floating>>,
        /// P14.12.
        pub p14_12: P14_12<Input<Floating>>,
        /// P14.13.
        pub p14_13: P14_13<Input<Floating>>,
        /// P14.14.
        pub p14_14: P14_14<Input<Floating>>,
        /// P14.15.
        pub p14_15: P14_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT14 {}

    impl PortExt for crate::pac::PORT14 {
        type Parts = PORT14Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT14Parts {
                p14_0: P14_0 { _mode: PhantomData },
                p14_1: P14_1 { _mode: PhantomData },
                p14_2: P14_2 { _mode: PhantomData },
                p14_3: P14_3 { _mode: PhantomData },
                p14_4: P14_4 { _mode: PhantomData },
                p14_5: P14_5 { _mode: PhantomData },
                p14_6: P14_6 { _mode: PhantomData },
                p14_7: P14_7 { _mode: PhantomData },
                p14_8: P14_8 { _mode: PhantomData },
                p14_9: P14_9 { _mode: PhantomData },
                p14_10: P14_10 { _mode: PhantomData },
                p14_11: P14_11 { _mode: PhantomData },
                p14_12: P14_12 { _mode: PhantomData },
                p14_13: P14_13 { _mode: PhantomData },
                p14_14: P14_14 { _mode: PhantomData },
                p14_15: P14_15 { _mode: PhantomData },
            };
            parts.p14_0.make_floating();
            parts.p14_1.make_floating();
            parts.p14_2.make_floating();
            parts.p14_3.make_floating();
            parts.p14_4.make_floating();
            parts.p14_5.make_floating();
            parts.p14_6.make_floating();
            parts.p14_7.make_floating();
            parts.p14_8.make_floating();
            parts.p14_9.make_floating();
            parts.p14_10.make_floating();
            parts.p14_11.make_floating();
            parts.p14_12.make_floating();
            parts.p14_13.make_floating();
            parts.p14_14.make_floating();
            parts.p14_15.make_floating();
            parts
        }
    }

    pin!(PORT14, P14_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin!(PORT14, P14_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin!(PORT14, P14_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin!(PORT14, P14_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin!(PORT14, P14_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin!(PORT14, P14_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin!(PORT14, P14_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin!(PORT14, P14_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin!(PORT14, P14_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin!(PORT14, P14_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin!(PORT14, P14_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin!(PORT14, P14_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin!(PORT14, P14_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin!(PORT14, P14_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin!(PORT14, P14_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin!(PORT14, P14_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);

    /// Pins of `PORT15`.
    pub struct PORT15Parts {
        /// P15.0.
        pub p15_0: P15_0<Input<Floating>>,
        /// P15.1.
        pub p15_1: P15_1<Input<Floating>>,
        /// P15.2.
        pub p15_2: P15_2<Input<Floating>>,
        /// P15.3.
        pub p15_3: P15_3<Input<Floating>>,
        /// P15.4.
        pub p15_4: P15_4<Input<Floating>>,
        /// P15.5.
        pub p15_5: P15_5<Input<Floating>>,
        /// P15.6.
        pub p15_6: P15_6<Input<Floating>>,
        /// P15.7.
        pub p15_7: P15_7<Input<Floating>>,
        /// P15.8.
        pub p15_8: P15_8<Input<Floating>>,
        /// P15.9.
        pub p15_9: P15_9<Input<Floating>>,
        /// P15.10.
        pub p15_10: P15_10<Input<Floating>>,
        /// P15.11.
        pub p15_11: P15_11<Input<Floating>>,
        /// P15.12.
        pub p15_12: P15_12<Input<Floating>>,
        /// P15.13.
        pub p15_13: P15_13<Input<Floating>>,
        /// P15.14.
        pub p15_14: P15_14<Input<Floating>>,
        /// P15.15.
        pub p15_15: P15_15<Input<Floating>>,
    }

    impl Sealed for crate::pac::PORT15 {}

    impl PortExt for crate::pac::PORT15 {
        type Parts = PORT15Parts;
        fn split(self) -> Self::Parts {
            let parts = PORT15Parts {
                p15_0: P15_0 { _mode: PhantomData },
                p15_1: P15_1 { _mode: PhantomData },
                p15_2: P15_2 { _mode: PhantomData },
                p15_3: P15_3 { _mode: PhantomData },
                p15_4: P15_4 { _mode: PhantomData },
                p15_5: P15_5 { _mode: PhantomData },
                p15_6: P15_6 { _mode: PhantomData },
                p15_7: P15_7 { _mode: PhantomData },
                p15_8: P15_8 { _mode: PhantomData },
                p15_9: P15_9 { _mode: PhantomData },
                p15_10: P15_10 { _mode: PhantomData },
                p15_11: P15_11 { _mode: PhantomData },
                p15_12: P15_12 { _mode: PhantomData },
                p15_13: P15_13 { _mode: PhantomData },
                p15_14: P15_14 { _mode: PhantomData },
                p15_15: P15_15 { _mode: PhantomData },
            };
            parts.p15_0.make_floating();
            parts.p15_1.make_floating();
            parts.p15_2.make_floating();
            parts.p15_3.make_floating();
            parts.p15_4.make_floating();
            parts.p15_5.make_floating();
            parts.p15_6.make_floating();
            parts.p15_7.make_floating();
            parts.p15_8.make_floating();
            parts.p15_9.make_floating();
            parts.p15_10.make_floating();
            parts.p15_11.make_floating();
            parts.p15_12.make_floating();
            parts.p15_13.make_floating();
            parts.p15_14.make_floating();
            parts.p15_15.make_floating();
            parts
        }
    }

    pin!(PORT15, P15_0, 0, iocr0, pc0, ps0, pr0, p0, hw0);
    pin!(PORT15, P15_1, 1, iocr0, pc1, ps1, pr1, p1, hw1);
    pin!(PORT15, P15_2, 2, iocr0, pc2, ps2, pr2, p2, hw2);
    pin!(PORT15, P15_3, 3, iocr0, pc3, ps3, pr3, p3, hw3);
    pin!(PORT15, P15_4, 4, iocr4, pc4, ps4, pr4, p4, hw4);
    pin!(PORT15, P15_5, 5, iocr4, pc5, ps5, pr5, p5, hw5);
    pin!(PORT15, P15_6, 6, iocr4, pc6, ps6, pr6, p6, hw6);
    pin!(PORT15, P15_7, 7, iocr4, pc7, ps7, pr7, p7, hw7);
    pin!(PORT15, P15_8, 8, iocr8, pc8, ps8, pr8, p8, hw8);
    pin!(PORT15, P15_9, 9, iocr8, pc9, ps9, pr9, p9, hw9);
    pin!(PORT15, P15_10, 10, iocr8, pc10, ps10, pr10, p10, hw10);
    pin!(PORT15, P15_11, 11, iocr8, pc11, ps11, pr11, p11, hw11);
    pin!(PORT15, P15_12, 12, iocr12, pc12, ps12, pr12, p12, hw12);
    pin!(PORT15, P15_13, 13, iocr12, pc13, ps13, pr13, p13, hw13);
    pin!(PORT15, P15_14, 14, iocr12, pc14, ps14, pr14, p14, hw14);
    pin!(PORT15, P15_15, 15, iocr12, pc15, ps15, pr15, p15, hw15);
}

#[cfg(any(feature = "xmc4700", feature = "xmc4800"))]
pub use large::*;
