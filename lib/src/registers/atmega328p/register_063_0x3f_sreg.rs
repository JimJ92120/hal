// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=10
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SREG;

impl Register for SREG {
    const ADDRESS: Address = (IO_OFFSET + 0x3F) as Address;
}

impl SREG {
    pub const C: Bit = Bit::Zero;
    pub const Z: Bit = Bit::One;
    pub const N: Bit = Bit::Two;
    pub const V: Bit = Bit::Three;
    pub const S: Bit = Bit::Four;
    pub const H: Bit = Bit::Five;
    pub const T: Bit = Bit::Six;
    pub const I: Bit = Bit::Seven;
}
