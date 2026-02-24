// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=112
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct ICR1H;

impl Register for ICR1H {
    const ADDRESS: Address = 0x87 as Address;
}

impl ICR1H {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
