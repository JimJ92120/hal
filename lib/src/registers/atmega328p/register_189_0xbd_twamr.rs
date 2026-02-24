// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=201
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TWAMR;

impl Register for TWAMR {
    const ADDRESS: Address = 0xBD as Address;
}

impl TWAMR {
    // 0
    pub const TWAM0: Bit = Bit::One;
    pub const TWAM1: Bit = Bit::Two;
    pub const TWAM2: Bit = Bit::Three;
    pub const TWAM3: Bit = Bit::Four;
    pub const TWAM4: Bit = Bit::Five;
    pub const TWAM5: Bit = Bit::Six;
    pub const TWAM6: Bit = Bit::Seven;
}
