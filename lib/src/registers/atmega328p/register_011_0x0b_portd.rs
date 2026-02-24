// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=73
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct PORTD;

impl Register for PORTD {
    const ADDRESS: Address = (IO_OFFSET + 0x0B) as Address;
}

impl PORTD {
    pub const PD0: Bit = Bit::Zero;
    pub const PD1: Bit = Bit::One;
    pub const PD2: Bit = Bit::Two;
    pub const PD3: Bit = Bit::Three;
    pub const PD4: Bit = Bit::Four;
    pub const PD5: Bit = Bit::Five;
    pub const PD6: Bit = Bit::Six;
    pub const PD7: Bit = Bit::Seven;
}
