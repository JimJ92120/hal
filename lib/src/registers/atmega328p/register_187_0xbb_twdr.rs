// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=200
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TWDR;

impl Register for TWDR {
    const ADDRESS: Address = 0xBB as Address;
}

impl TWDR {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
