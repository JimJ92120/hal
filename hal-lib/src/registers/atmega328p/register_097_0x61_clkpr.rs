// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=33
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct CLKPR;

impl Register for CLKPR {
    const ADDRESS: Address = 0x61 as Address;
}

impl CLKPR {
    pub const CLKPS0: Bit = Bit::Zero;
    pub const CLKPS1: Bit = Bit::One;
    pub const CLKPS2: Bit = Bit::Two;
    pub const CLKPS3: Bit = Bit::Three;
    // 4
    // 5
    // 6
    pub const CLKPCE: Bit = Bit::Seven;
}
