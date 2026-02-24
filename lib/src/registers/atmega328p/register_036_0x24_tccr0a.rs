// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TCCR0A;

impl Register for TCCR0A {
    const ADDRESS: Address = (IO_OFFSET + 0x24) as Address;
}

impl TCCR0A {
    pub const WGM00: Bit = Bit::Zero;
    pub const WGM01: Bit = Bit::One;
    // 2
    // 3
    pub const COM0B0: Bit = Bit::Four;
    pub const COM0B1: Bit = Bit::Five;
    pub const COM0A0: Bit = Bit::Six;
    pub const COM0A1: Bit = Bit::Seven;
}
