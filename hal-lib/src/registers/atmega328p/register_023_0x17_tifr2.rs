// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=132
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TIFR2;

impl Register for TIFR2 {
    const ADDRESS: Address = (IO_OFFSET + 0x17) as Address;
}

impl TIFR2 {
    pub const TOV2: Bit = Bit::Zero;
    pub const OCF2A: Bit = Bit::One;
    pub const OCF2B: Bit = Bit::Two;
    // 3
    // 4
    // 5
    // 6
    // 7
}
