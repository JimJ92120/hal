// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=131
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct OCR2B;

impl Register for OCR2B {
    const ADDRESS: Address = 0xB4 as Address;
}

