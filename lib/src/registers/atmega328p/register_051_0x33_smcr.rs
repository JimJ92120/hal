// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=35
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SMCR;

impl Register for SMCR {
    const ADDRESS: Address = (IO_OFFSET + 0x33) as Address;
}

impl SMCR {
    pub const SE: Bit = Bit::Zero;
    pub const SM0: Bit = Bit::One;
    pub const SM1: Bit = Bit::Two;
    pub const SM2: Bit = Bit::Three;
    // 4
    // 5
    // 6
    // 7
}
