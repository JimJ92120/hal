// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=111
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct OCR1BL;

impl Register for OCR1BL {
    const ADDRESS: Address = 0x8A as Address;
}
