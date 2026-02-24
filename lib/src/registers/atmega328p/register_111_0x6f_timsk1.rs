// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=112
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TIMSK1;

impl Register for TIMSK1 {
    const ADDRESS: Address = 0x6F as Address;
}

impl TIMSK1 {
    pub const TOIE1: Bit = Bit::Zero;
    pub const OCIE1A: Bit = Bit::One;
    pub const OCIE1B: Bit = Bit::Two;
    // 3
    // 4
    pub const ICIE1: Bit = Bit::Five;
    // 6
    // 7
}
