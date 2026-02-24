// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=88
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TIMSK0;

impl Register for TIMSK0 {
    const ADDRESS: Address = 0x6E as Address;
}

impl TIMSK0 {
    pub const TOIE0: Bit = Bit::Zero;
    pub const OCIE0A: Bit = Bit::One;
    pub const OCIE0B: Bit = Bit::Two;
    // 3
    // 4
    // 5
    // 6
    // 7
}
