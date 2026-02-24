// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=218
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct ADCSRA;

impl Register for ADCSRA {
    const ADDRESS: Address = 0x7A as Address;
}

impl ADCSRA {
    pub const ADPS0: Bit = Bit::Zero;
    pub const ADPS1: Bit = Bit::One;
    pub const ADPS2: Bit = Bit::Two;
    pub const ADIE: Bit = Bit::Three;
    pub const ADIF: Bit = Bit::Four;
    pub const ADATE: Bit = Bit::Five;
    pub const ADSC: Bit = Bit::Six;
    pub const ADEN: Bit = Bit::Seven;
}
