// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=57
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct PCMSK1;

impl Register for PCMSK1 {
    const ADDRESS: Address = 0x6C as Address;
}

impl PCMSK1 {
    pub const PCINT8: Bit = Bit::Zero;
    pub const PCINT9: Bit = Bit::One;
    pub const PCINT10: Bit = Bit::Two;
    pub const PCINT11: Bit = Bit::Three;
    pub const PCINT12: Bit = Bit::Four;
    pub const PCINT13: Bit = Bit::Five;
    pub const PCINT14: Bit = Bit::Six;
    // 7
}
