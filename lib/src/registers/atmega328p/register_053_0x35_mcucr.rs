// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=38
// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=52
// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=72
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct MCUCR;

impl Register for MCUCR {
    const ADDRESS: Address = (IO_OFFSET + 0x35) as Address;
}

impl MCUCR {
    pub const IVCE: Bit = Bit::Zero;
    pub const IVSEL: Bit = Bit::One;
    // 2
    // 3
    pub const PUD: Bit = Bit::Four;
    pub const BODSE: Bit = Bit::Five;
    pub const BODS: Bit = Bit::Six;
    // 7
}
