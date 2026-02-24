// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=142
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPDR;

impl Register for SPDR {
    const ADDRESS: Address = (IO_OFFSET + 0x2E) as Address;
}

impl SPDR {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
