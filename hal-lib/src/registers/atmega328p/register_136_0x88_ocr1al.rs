// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=111
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct OCR1AL;

impl Register for OCR1AL {
    const ADDRESS: Address = 0x88 as Address;
}

impl OCR1AL {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
