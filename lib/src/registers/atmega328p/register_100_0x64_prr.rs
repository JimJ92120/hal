// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=36
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct PRR;

impl Register for PRR {
    const ADDRESS: Address = 0x64 as Address;
}

impl PRR {
    pub const PRADC: Bit = Bit::Zero;
    pub const PRUSAR0: Bit = Bit::One;
    pub const PRSPI: Bit = Bit::Two;
    pub const PRTIM1: Bit = Bit::Three;
    // 4
    pub const PRTIM0: Bit = Bit::Five;
    pub const PRTIM2: Bit = Bit::Six;
    pub const PRTWI: Bit = Bit::Seven;
}
