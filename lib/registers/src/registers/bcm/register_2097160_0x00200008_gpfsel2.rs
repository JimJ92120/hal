// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=91
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct GPFSEL2;

impl Register<u32> for GPFSEL2 {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x00200008) as *mut u32;
}

// GPIO 20-29
#[derive(Debug)]
#[repr(u32)]
pub enum GPFSEL2BitField {
    FSEL20 = 0,
    // 1 (FSEL20)
    // 2 (FSEL20)
    FSEL21 = 3,
    // 4 (FSEL21)
    // 5 (FSEL21)
    FSEL22 = 6,
    // 7 (FSEL22)
    // 8 (FSEL22)
    FSEL23 = 9,
    // 10 (FSEL23)
    // 11 (FSEL23)
    FSEL24 = 12,
    // 13 (FSEL24)
    // 14 (FSEL24)
    FSEL25 = 15,
    // 16 (FSEL25)
    // 17 (FSEL25)
    FSEL26 = 18,
    // 19 (FSEL26)
    // 20 (FSEL27)
    FSEL27 = 21,
    // 22 (FSEL27)
    // 23 (FSEL27)
    FSEL28 = 24,
    // 25 (FSEL28)
    // 26 (FSEL28)
    FSEL29 = 27,
    // 28 (FSEL29)
    // 29 (FSEL29)
    // 30
    // 31
}
