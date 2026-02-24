// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=55
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct EIFR;

impl Register for EIFR {
    const ADDRESS: Address = (IO_OFFSET + 0x1C) as Address;
}

impl EIFR {
    pub const INTF0: Bit = Bit::Zero;
    pub const INTF1: Bit = Bit::One;
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
