// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=200
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TWSR;

impl Register for TWSR {
    const ADDRESS: Address = 0xB9 as Address;
}

impl TWSR {
    pub const TWPS0: Bit = Bit::Zero;
    pub const TWPS1: Bit = Bit::One;
    // 2
    pub const TWS3: Bit = Bit::Three;
    pub const TWS4: Bit = Bit::Four;
    pub const TWS5: Bit = Bit::Five;
    pub const TWS6: Bit = Bit::Six;
    pub const TWS7: Bit = Bit::Seven;
}
