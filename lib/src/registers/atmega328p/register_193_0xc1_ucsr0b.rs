// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=160
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct UCSR0B;

impl Register for UCSR0B {
    const ADDRESS: Address = 0xC1 as Address;
}

impl UCSR0B {
    pub const TXB80: Bit = Bit::Zero;
    pub const RXB80: Bit = Bit::One;
    pub const UCSZ02: Bit = Bit::Two;
    pub const TXEN0: Bit = Bit::Three;
    pub const RXEN0: Bit = Bit::Four;
    pub const UDRIE0: Bit = Bit::Five;
    pub const TXCIE0: Bit = Bit::Six;
    pub const RXCIE0: Bit = Bit::Seven;
}
