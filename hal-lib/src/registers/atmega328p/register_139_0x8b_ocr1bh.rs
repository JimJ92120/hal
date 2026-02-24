// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=111
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct OCR1BH;

impl Register for OCR1BH {
    const ADDRESS: Address = 0x8B as Address;
}

impl OCR1BH {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
