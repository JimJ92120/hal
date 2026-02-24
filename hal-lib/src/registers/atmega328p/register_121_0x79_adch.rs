// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=219
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct ADCH;

impl Register for ADCH {
    const ADDRESS: Address = 0x79 as Address;
}

impl ADCH {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
