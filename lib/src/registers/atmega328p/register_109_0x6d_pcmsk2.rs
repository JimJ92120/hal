// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=57
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct PCMSK2;

impl Register for PCMSK2 {
    const ADDRESS: Address = 0x6D as Address;
}

impl PCMSK2 {
    pub const PCINT16: Bit = Bit::Zero;
    pub const PCINT17: Bit = Bit::One;
    pub const PCINT18: Bit = Bit::Two;
    pub const PCINT19: Bit = Bit::Three;
    pub const PCINT20: Bit = Bit::Four;
    pub const PCINT21: Bit = Bit::Five;
    pub const PCINT22: Bit = Bit::Six;
    pub const PCINT23: Bit = Bit::Seven;
}
