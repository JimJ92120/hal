// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=162
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct UBRR0L;

impl Register for UBRR0L {
    const ADDRESS: Address = 0xC4 as Address;
}

impl UBRR0L {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
