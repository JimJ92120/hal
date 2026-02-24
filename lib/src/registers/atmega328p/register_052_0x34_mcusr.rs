// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=46
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct MCUSR;

impl Register for MCUSR {
    const ADDRESS: Address = (IO_OFFSET + 0x34) as Address;
}

impl MCUSR {
    pub const PORF: Bit = Bit::Zero;
    pub const EXTRF: Bit = Bit::One;
    pub const BORF: Bit = Bit::Two;
    pub const WDRF: Bit = Bit::Three;
    // 4
    // 5
    // 6
    // 7
}
