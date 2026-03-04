// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=91
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct GPFSEL1;

impl Register<u32> for GPFSEL1 {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x00200004) as *mut u32;
}

// GPIO 10-19
#[derive(Debug)]
#[repr(u32)]
pub enum GPFSEL1BitField {
    FSEL10 = 0,
    // 1 (FSEL10)
    // 2 (FSEL10)
    FSEL11 = 3,
    // 4 (FSEL11)
    // 5 (FSEL11)
    FSEL12 = 6,
    // 7 (FSEL12)
    // 8 (FSEL12)
    FSEL13 = 9,
    // 10 (FSEL13)
    // 11 (FSEL13)
    FSEL14 = 12,
    // 13 (FSEL14)
    // 14 (FSEL14)
    FSEL15 = 15,
    // 16 (FSEL15)
    // 17 (FSEL15)
    FSEL16 = 18,
    // 19 (FSEL16)
    // 20 (FSEL17)
    FSEL17 = 21,
    // 22 (FSEL17)
    // 23 (FSEL17)
    FSEL18 = 24,
    // 25 (FSEL18)
    // 26 (FSEL18)
    FSEL19 = 27,
    // 28 (FSEL19)
    // 29 (FSEL19)
    // 30
    // 31
}
