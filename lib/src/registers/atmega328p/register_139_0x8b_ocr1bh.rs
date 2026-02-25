// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=111
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct OCR1BH;

impl Register for OCR1BH {
    const ADDRESS: Address = 0x8B as Address;
}
