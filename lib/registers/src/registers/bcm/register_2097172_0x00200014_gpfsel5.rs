// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=91
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct GPFSEL5;

impl Register<u32> for GPFSEL5 {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x00200014) as *mut u32;
}

// GPIO 50-59
#[derive(Debug)]
#[repr(u32)]
pub enum GPFSEL5BitField {
    FSEL50 = 0,
    // 1 (FSEL50)
    // 2 (FSEL50)
    FSEL51 = 3,
    // 4 (FSEL51)
    // 5 (FSEL51)
    FSEL52 = 6,
    // 7 (FSEL52)
    // 8 (FSEL52)
    FSEL53 = 9,
    // 10 (FSEL53)
    // 11 (FSEL53)
    FSEL54 = 12,
    // 13 (FSEL54)
    // 14 (FSEL54)
    FSEL55 = 15,
    // 16 (FSEL55)
    // 17 (FSEL55)
    FSEL56 = 18,
    // 19 (FSEL56)
    // 20 (FSEL57)
    FSEL57 = 21,
    // 22 (FSEL57)
    // 23 (FSEL57)
    FSEL58 = 24,
    // 25 (FSEL58)
    // 26 (FSEL58)
    FSEL59 = 27,
    // 28 (FSEL59)
    // 29 (FSEL59)
    // 30
    // 31
}
