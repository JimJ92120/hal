// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=91
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct GPFSEL0;

impl Register<u32> for GPFSEL0 {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x00200000) as *mut u32;
}

// GPIO 0-9
#[derive(Debug)]
#[repr(u32)]
pub enum GPFSEL0BitField {
    FSEL0 = 0,
    // 1 (FSEL0)
    // 2 (FSEL0)
    FSEL1 = 3,
    // 4 (FSEL1)
    // 5 (FSEL1)
    FSEL2 = 6,
    // 7 (FSEL2)
    // 8 (FSEL2)
    FSEL3 = 9,
    // 10 (FSEL3)
    // 11 (FSEL3)
    FSEL4 = 12,
    // 13 (FSEL4)
    // 14 (FSEL4)
    FSEL5 = 15,
    // 16 (FSEL5)
    // 17 (FSEL5)
    FSEL6 = 18,
    // 19 (FSEL6)
    // 20 (FSEL7)
    FSEL7 = 21,
    // 22 (FSEL7)
    // 23 (FSEL7)
    FSEL8 = 24,
    // 25 (FSEL8)
    // 26 (FSEL8)
    FSEL9 = 27,
    // 28 (FSEL9)
    // 29 (FSEL9)
    // 30
    // 31
}
