// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=108
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TCCR1A;

impl Register for TCCR1A {
    const ADDRESS: Address = 0x80 as Address;
}

impl TCCR1A {
    pub const WGM10: Bit = Bit::Zero;
    pub const WGM11: Bit = Bit::One;
    // 2
    // 3
    pub const COM1B0: Bit = Bit::Four;
    pub const COM1B1: Bit = Bit::Five;
    pub const COM1A0: Bit = Bit::Six;
    pub const COM1A1: Bit = Bit::Seven;
}
