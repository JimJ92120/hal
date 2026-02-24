// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=217
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct ADMUX;

impl Register for ADMUX {
    const ADDRESS: Address = 0x7C as Address;
}

impl ADMUX {
    pub const MUX0: Bit = Bit::Zero;
    pub const MUX1: Bit = Bit::One;
    pub const MUX2: Bit = Bit::Two;
    pub const MUX3: Bit = Bit::Three;
    // 4
    pub const ADLAR: Bit = Bit::Five;
    pub const REFS0: Bit = Bit::Six;
    pub const REFS1: Bit = Bit::Seven;
}
