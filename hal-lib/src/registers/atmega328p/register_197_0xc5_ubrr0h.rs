// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=162
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct UBRR0H;

impl Register for UBRR0H {
    const ADDRESS: Address = 0xC5 as Address;
}

impl UBRR0H {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
