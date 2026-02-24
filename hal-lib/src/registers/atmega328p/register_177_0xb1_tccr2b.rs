// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=130
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TCCR2B;

impl Register for TCCR2B {
    const ADDRESS: Address = 0xB1 as Address;
}

impl TCCR2B {
    pub const CS20: Bit = Bit::Zero;
    pub const CS21: Bit = Bit::One;
    pub const CS22: Bit = Bit::Two;
    pub const WGM22: Bit = Bit::Three;
    // 4
    // 5
    pub const FOC2B: Bit = Bit::Six;
    pub const FOC2A: Bit = Bit::Seven;
}
