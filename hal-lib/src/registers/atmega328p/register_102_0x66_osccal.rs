// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=32
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct OSCCAL;

impl Register for OSCCAL {
    const ADDRESS: Address = 0x66 as Address;
}

impl OSCCAL {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
