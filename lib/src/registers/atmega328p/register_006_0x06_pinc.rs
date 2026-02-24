// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=73
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct PINC;

impl Register for PINC {
    const ADDRESS: Address = (IO_OFFSET + 0x06) as Address;
}

impl PINC {
    pub const AC0: Bit = Bit::Zero;
    pub const AC1: Bit = Bit::One;
    pub const AC2: Bit = Bit::Two;
    pub const AC3: Bit = Bit::Three;
    pub const AC4: Bit = Bit::Four;
    pub const AC5: Bit = Bit::Five;
    // 6
    // 7
}
