// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=198
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TWBR;

impl Register for TWBR {
    const ADDRESS: Address = 0xB8 as Address;
}

impl TWBR {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
