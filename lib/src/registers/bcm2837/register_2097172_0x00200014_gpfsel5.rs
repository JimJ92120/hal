// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=72
use crate::globals::{ Register, Address };
use super::MMIO_BASE_OFFSET;

#[derive(Debug)]
pub struct GPFSEL5;

impl Register for GPFSEL5 {
    const ADDRESS: Address = (MMIO_BASE_OFFSET + 0x00200014) as Address;
}

// GPIO 50-59
#[derive(Debug)]
#[repr(u32)]
pub enum GPFSEL5BitField {
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
