// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TCCR0B;

impl Register for TCCR0B {
    const ADDRESS: Address = (IO_OFFSET + 0x25) as Address;
}

impl TCCR0B {
    pub const CS00: Bit = Bit::Zero;
    pub const CS01: Bit = Bit::One;
    pub const CS02: Bit = Bit::Two;
    pub const WGM02: Bit = Bit::Three;
    // 4
    // 5
    pub const FOC0B: Bit = Bit::Six;
    pub const FOC0A: Bit = Bit::Seven;
}
