// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=115
// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=134
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct GTCCR;

impl Register for GTCCR {
    const ADDRESS: Address = (IO_OFFSET + 0x23) as Address;
}

impl GTCCR {
    pub const PSRSYNC: Bit = Bit::Zero;
    pub const PSRASY: Bit = Bit::One;
    // 2
    // 3
    // 4
    // 5
    // 6
    pub const TSM: Bit = Bit::Seven;
}
