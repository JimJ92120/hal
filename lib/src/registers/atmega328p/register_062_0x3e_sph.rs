// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=13
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPH;

impl Register for SPH {
    const ADDRESS: Address = (IO_OFFSET + 0x3E) as Address;
}

impl SPH {
    pub const SP8: Bit = Bit::Zero;
    pub const SP9: Bit = Bit::One;
    pub const SP10: Bit = Bit::Two;
    // 3
    // 4
    // 5
    // 6
    // 7
}
