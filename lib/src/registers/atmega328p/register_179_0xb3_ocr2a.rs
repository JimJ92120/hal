// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=131
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct OCR2A;

impl Register for OCR2A {
    const ADDRESS: Address = 0xB3 as Address;
}

impl OCR2A {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
