// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=57
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct PCMSK0;

impl Register for PCMSK0 {
    const ADDRESS: Address = 0x6B as Address;
}

impl PCMSK0 {
    pub const PCINT0: Bit = Bit::Zero;
    pub const PCINT1: Bit = Bit::One;
    pub const PCINT2: Bit = Bit::Two;
    pub const PCINT3: Bit = Bit::Three;
    pub const PCINT4: Bit = Bit::Four;
    pub const PCINT5: Bit = Bit::Five;
    pub const PCINT6: Bit = Bit::Six;
    pub const PCINT7: Bit = Bit::Seven;
}
