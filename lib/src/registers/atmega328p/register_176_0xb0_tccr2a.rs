// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=127
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TCCR2A;

impl Register for TCCR2A {
    const ADDRESS: Address = 0xB0 as Address;
}

impl TCCR2A {
    pub const WGM20: Bit = Bit::Zero;
    pub const WGM21: Bit = Bit::One;
    // 2
    // 3
    pub const COM2B0: Bit = Bit::Four;
    pub const COM2B1: Bit = Bit::Five;
    pub const COM2A0: Bit = Bit::Six;
    pub const COM2A1: Bit = Bit::Seven;
}
