// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=72
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct DDRB;

impl Register for DDRB {
    const ADDRESS: Address = (IO_OFFSET + 0x04) as Address;
}

impl DDRB {
    pub const PB0: Bit = Bit::Zero;
    pub const PB1: Bit = Bit::One;
    pub const PB2: Bit = Bit::Two;
    pub const PB3: Bit = Bit::Three;
    pub const PB4: Bit = Bit::Four;
    pub const PB5: Bit = Bit::Five;
    // 6
    // 7
}
