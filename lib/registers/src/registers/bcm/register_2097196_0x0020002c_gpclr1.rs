// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=95
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct GPCLR1;

impl Register<u32> for GPCLR1 {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x0020002C) as *mut u32;
}

// GPIO 32-53
#[derive(Debug)]
#[repr(u32)]
pub enum GPCLR1BitField {
    PIN32 = 0,
    PIN33 = 1,
    PIN34 = 2,
    PIN35 = 3,
    PIN36 = 4,
    PIN37 = 5,
    PIN38 = 6,
    PIN39 = 7,
    PIN40 = 8,
    PIN41 = 9,
    PIN42 = 10,
    PIN43 = 11,
    PIN44 = 12,
    PIN45 = 13,
    PIN46 = 14,
    PIN47 = 15,
    PIN48 = 16,
    PIN49 = 17,
    PIN50 = 18,
    PIN51 = 19,
    PIN52 = 20,
    PIN53 = 21,
    // PIN54 = 22,
    // PIN55 = 23,
    // PIN56 = 24,
    // PIN57 = 25,
    // PIN58 = 26,
    // PIN59 = 27,
    // PIN60 = 28,
    // PIN61 = 29,
    // PIN62 = 30,
    // PIN63 = 31,
}
