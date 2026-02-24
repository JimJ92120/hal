// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=159
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct UDR0;

impl Register for UDR0 {
    const ADDRESS: Address = 0xC6 as Address;
}

impl UDR0 {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
