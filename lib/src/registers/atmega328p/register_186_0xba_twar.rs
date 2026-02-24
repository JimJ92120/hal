// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=201
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TWAR;

impl Register for TWAR {
    const ADDRESS: Address = 0xBA as Address;
}

impl TWAR {
    pub const TWGCE: Bit = Bit::Zero;
    pub const TWA0: Bit = Bit::One;
    pub const TWA1: Bit = Bit::Two;
    pub const TWA2: Bit = Bit::Three;
    pub const TWA3: Bit = Bit::Four;
    pub const TWA4: Bit = Bit::Five;
    pub const TWA5: Bit = Bit::Six;
    pub const TWA6: Bit = Bit::Seven;
}
