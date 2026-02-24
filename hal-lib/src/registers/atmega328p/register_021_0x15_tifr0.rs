// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TIFR0;

impl Register for TIFR0 {
    const ADDRESS: Address = (IO_OFFSET + 0x15) as Address;
}

impl TIFR0 {
    pub const TOV0: Bit = Bit::Zero;
    pub const OCF0A: Bit = Bit::One;
    pub const OCF0B: Bit = Bit::Two;
    // 3
    // 4
    // 5
    // 6
    // 7
}
