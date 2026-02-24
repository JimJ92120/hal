// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=199
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TWCR;

impl Register for TWCR {
    const ADDRESS: Address = 0xBC as Address;
}

impl TWCR {
    pub const TWIE: Bit = Bit::Zero;
    // 2
    pub const TWEN: Bit = Bit::Two;
    pub const TWWC: Bit = Bit::Three;
    pub const TWSTO: Bit = Bit::Four;
    pub const TWSTA: Bit = Bit::Five;
    pub const TWEA: Bit = Bit::Six;
    pub const TWINT: Bit = Bit::Seven;
}
