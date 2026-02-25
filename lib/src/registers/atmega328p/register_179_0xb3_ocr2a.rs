// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=131
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct OCR2A;

impl Register for OCR2A {
    const ADDRESS: Address = 0xB3 as Address;
}
