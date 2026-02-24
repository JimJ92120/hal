// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=161
// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=172
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct UCSR0C;

impl Register for UCSR0C {
    const ADDRESS: Address = 0xC2 as Address;
}

impl UCSR0C {
    pub const UCPOL0: Bit = Bit::Zero;
    pub const UCSZ00: Bit = Bit::One;
    pub const UCSZ01: Bit = Bit::Two;
    pub const USBS0: Bit = Bit::Three;
    pub const UPM00: Bit = Bit::Four;
    pub const UPM01: Bit = Bit::Five;
    pub const UMSEL00: Bit = Bit::Six;
    pub const UMSEL01: Bit = Bit::Seven;

    // alternative naming
    pub const UCPHA0: Bit = Bit::One;
    pub const UDORD0: Bit = Bit::Two;
}
