// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=113
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TIFR1;

impl Register for TIFR1 {
    const ADDRESS: Address = (IO_OFFSET + 0x16) as Address;
}

impl TIFR1 {
    pub const TOV1: Bit = Bit::Zero;
    pub const OCF1A: Bit = Bit::One;
    pub const OCF1B: Bit = Bit::Two;
    // 3
    // 4
    // 5
    // 6
    // 7
}
