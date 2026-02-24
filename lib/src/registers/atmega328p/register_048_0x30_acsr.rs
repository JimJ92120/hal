// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=203
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct ACSR;

impl Register for ACSR {
    const ADDRESS: Address = (IO_OFFSET + 0x30) as Address;
}

impl ACSR {
    pub const ACIS0: Bit = Bit::Zero;
    pub const ACIS1: Bit = Bit::One;
    pub const ACIC: Bit = Bit::Two;
    pub const ACIE: Bit = Bit::Three;
    pub const ACI: Bit = Bit::Four;
    pub const ACO: Bit = Bit::Five;
    pub const ACBG: Bit = Bit::Six;
    pub const ACD: Bit = Bit::Seven;
}
