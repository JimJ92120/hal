// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Bit, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct OCR0B;

impl Register for OCR0B {
    const ADDRESS: Address = (IO_OFFSET + 0x28) as Address;
}

impl OCR0B {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
