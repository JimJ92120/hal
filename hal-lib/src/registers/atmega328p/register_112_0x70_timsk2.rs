// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=132
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TIMSK2;

impl Register for TIMSK2 {
    const ADDRESS: Address = 0x70 as Address;
}

impl TIMSK2 {
    pub const TOIE2: Bit = Bit::Zero;
    pub const OCIE2A: Bit = Bit::One;
    pub const OCIE2B: Bit = Bit::Two;
    // 3
    // 4
    // 5
    // 6
    // 7
}
