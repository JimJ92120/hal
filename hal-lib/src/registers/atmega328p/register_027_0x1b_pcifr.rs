// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct PCIFR;

impl Register for PCIFR {
    const ADDRESS: Address = (IO_OFFSET + 0x1B) as Address;
}

impl PCIFR {
    pub const PCIF0: Bit = Bit::Zero;
    pub const PCIF1: Bit = Bit::One;
    pub const PCIF2: Bit = Bit::Two;
    // 3
    // 4
    // 5
    // 6
    // 7
}
