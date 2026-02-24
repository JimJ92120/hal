// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=23
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct GPIOR1;

impl Register for GPIOR1 {
    const ADDRESS: Address = (IO_OFFSET + 0x2A) as Address;
}

impl GPIOR1 {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
