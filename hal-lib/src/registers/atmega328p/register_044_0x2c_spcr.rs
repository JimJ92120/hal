// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=140
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPCR;

impl Register for SPCR {
    const ADDRESS: Address = (IO_OFFSET + 0x2C) as Address;
}

impl SPCR {
    pub const SPR0: Bit = Bit::Zero;
    pub const SPR1: Bit = Bit::One;
    pub const CPHA: Bit = Bit::Two;
    pub const CPOL: Bit = Bit::Three;
    pub const MSTR: Bit = Bit::Four;
    pub const DORD: Bit = Bit::Five;
    pub const SPE: Bit = Bit::Six;
    pub const SPIE: Bit = Bit::Seven;
}
