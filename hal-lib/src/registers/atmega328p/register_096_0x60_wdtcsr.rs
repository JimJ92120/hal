// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=47
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct WDTCSR;

impl Register for WDTCSR {
    const ADDRESS: Address = 0x60 as Address;
}

impl WDTCSR {
    pub const WDP0: Bit = Bit::Zero;
    pub const WDP1: Bit = Bit::One;
    pub const WDP2: Bit = Bit::Two;
    pub const WDE: Bit = Bit::Three;
    pub const WDCE: Bit = Bit::Four;
    pub const WDP3: Bit = Bit::Five;
    pub const WDIE: Bit = Bit::Six;
    pub const WDIF: Bit = Bit::Seven;
}
