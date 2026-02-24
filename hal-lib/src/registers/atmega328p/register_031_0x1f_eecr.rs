// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=20
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct EECR;

impl Register for EECR {
    const ADDRESS: Address = (IO_OFFSET + 0x1F) as Address;
}

impl EECR {
    pub const EERE: Bit = Bit::Zero;
    pub const EEPE: Bit = Bit::One;
    pub const EEMPE: Bit = Bit::Two;
    pub const EERIE: Bit = Bit::Three;
    pub const EEPM0: Bit = Bit::Four;
    pub const EEPM1: Bit = Bit::Five;
    // 6
    // 7
}
