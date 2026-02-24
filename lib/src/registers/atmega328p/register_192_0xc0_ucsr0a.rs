// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=159
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct UCSR0A;

impl Register for UCSR0A {
    const ADDRESS: Address = 0xC0 as Address;
}

impl UCSR0A {
    pub const MPCM0: Bit = Bit::Zero;
    pub const U2X0: Bit = Bit::One;
    pub const UPE0: Bit = Bit::Two;
    pub const DOR0: Bit = Bit::Three;
    pub const FE0: Bit = Bit::Four;
    pub const UDRE0: Bit = Bit::Five;
    pub const TXC0: Bit = Bit::Six;
    pub const RXC0: Bit = Bit::Seven;
}
