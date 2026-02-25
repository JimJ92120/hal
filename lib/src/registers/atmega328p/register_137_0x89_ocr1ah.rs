// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=111
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct OCR1AH;

impl Register for OCR1AH {
    const ADDRESS: Address = 0x89 as Address;
}
