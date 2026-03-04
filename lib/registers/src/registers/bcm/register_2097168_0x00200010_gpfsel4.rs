// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=91
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct GPFSEL4;

impl Register<u32> for GPFSEL4 {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x00200010) as *mut u32;
}

// GPIO 40-49
#[derive(Debug)]
#[repr(u32)]
pub enum GPFSEL4BitField {
    FSEL40 = 0,
    // 1 (FSEL40)
    // 2 (FSEL40)
    FSEL41 = 3,
    // 4 (FSEL41)
    // 5 (FSEL41)
    FSEL42 = 6,
    // 7 (FSEL42)
    // 8 (FSEL42)
    FSEL43 = 9,
    // 10 (FSEL43)
    // 11 (FSEL43)
    FSEL44 = 12,
    // 13 (FSEL44)
    // 14 (FSEL44)
    FSEL45 = 15,
    // 16 (FSEL45)
    // 17 (FSEL45)
    FSEL46 = 18,
    // 19 (FSEL46)
    // 20 (FSEL47)
    FSEL47 = 21,
    // 22 (FSEL47)
    // 23 (FSEL47)
    FSEL48 = 24,
    // 25 (FSEL48)
    // 26 (FSEL48)
    FSEL49 = 27,
    // 28 (FSEL49)
    // 29 (FSEL49)
    // 30
    // 31
}
