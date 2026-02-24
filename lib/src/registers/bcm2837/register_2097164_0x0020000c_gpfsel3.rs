// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=72
use crate::globals::{ Register, Bit, Address };
use super::MMIO_BASE_OFFSET;

#[derive(Debug)]
pub struct GPFSEL3;

impl Register for GPFSEL3 {
    const ADDRESS: Address = (MMIO_BASE_OFFSET + 0x0020000C) as Address;
}

// GPIO 30-39
impl GPFSEL3 {
    pub const FSEL0: Bit = Bit::Zero;
    // 1 (FSEL0)
    // 2 (FSEL0)
    pub const FSEL1: Bit = Bit::Three;
    // 4 (FSEL1)
    // 5 (FSEL1)
    pub const FSEL2: Bit = Bit::Six;
    // 7 (FSEL2)
    // 8 (FSEL2)
    pub const FSEL3: Bit = Bit::Nine;
    // 10 (FSEL3)
    // 11 (FSEL3)
    pub const FSEL4: Bit = Bit::Twelve;
    // 13 (FSEL4)
    // 14 (FSEL4)
    pub const FSEL5: Bit = Bit::Fifteen;
    // 16 (FSEL5)
    // 17 (FSEL5)
    pub const FSEL6: Bit = Bit::Eighteen;
    // 19 (FSEL6)
    // 20 (FSEL7)
    pub const FSEL7: Bit = Bit::TwentyOne;
    // 22 (FSEL7)
    // 23 (FSEL7)
    pub const FSEL8: Bit = Bit::TwentyFour;
    // 25 (FSEL8)
    // 26 (FSEL8)
    pub const FSEL9: Bit = Bit::TwentySeven;
    // 28 (FSEL9)
    // 29 (FSEL9)
    // 30
    // 31
}
