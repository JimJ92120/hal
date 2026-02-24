// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=23
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct GPIOR0;

impl Register for GPIOR0 {
    const ADDRESS: Address = (IO_OFFSET + 0x1E) as Address;
}

impl GPIOR0 {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
