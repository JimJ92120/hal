// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct PCICR;

impl Register for PCICR {
    const ADDRESS: Address = 0x68 as Address;
}

impl PCICR {
    pub const PCIE0: Bit = Bit::Zero;
    pub const PCIE1: Bit = Bit::One;
    pub const PCIE2: Bit = Bit::Two;
    // 3
    // 4
    // 5
    // 6
    // 7
}
