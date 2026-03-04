// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=91
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct GPFSEL3;

impl Register<u32> for GPFSEL3 {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x0020000C) as *mut u32;
}

// GPIO 30-39
#[derive(Debug)]
#[repr(u32)]
pub enum GPFSEL3BitField {
    FSEL30 = 0,
    // 1 (FSEL30)
    // 2 (FSEL30)
    FSEL31 = 3,
    // 4 (FSEL31)
    // 5 (FSEL31)
    FSEL32 = 6,
    // 7 (FSEL32)
    // 8 (FSEL32)
    FSEL33 = 9,
    // 10 (FSEL33)
    // 11 (FSEL33)
    FSEL34 = 12,
    // 13 (FSEL34)
    // 14 (FSEL34)
    FSEL35 = 15,
    // 16 (FSEL35)
    // 17 (FSEL35)
    FSEL36 = 18,
    // 19 (FSEL36)
    // 20 (FSEL37)
    FSEL37 = 21,
    // 22 (FSEL37)
    // 23 (FSEL37)
    FSEL38 = 24,
    // 25 (FSEL38)
    // 26 (FSEL38)
    FSEL39 = 27,
    // 28 (FSEL39)
    // 29 (FSEL39)
    // 30
    // 31
}
