// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=220
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct ADCSRB;

impl Register for ADCSRB {
    const ADDRESS: Address = 0x7B as Address;
}

impl ADCSRB {
    pub const ADTS0: Bit = Bit::Zero;
    pub const ADTS1: Bit = Bit::One;
    pub const ADTS2: Bit = Bit::Two;
    // 3
    // 4
    // 5
    pub const ACME: Bit = Bit::Six;
    // 7
}
