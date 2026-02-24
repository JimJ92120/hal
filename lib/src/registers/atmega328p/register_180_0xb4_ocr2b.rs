// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=131
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct OCR2B;

impl Register for OCR2B {
    const ADDRESS: Address = 0xB4 as Address;
}

impl OCR2B {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
