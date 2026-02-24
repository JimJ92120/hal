// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=20
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct EEARH;

impl Register for EEARH {
    const ADDRESS: Address = (IO_OFFSET + 0x22) as Address;
}

impl EEARH {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
