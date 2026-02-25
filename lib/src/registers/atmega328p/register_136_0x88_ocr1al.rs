// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=111
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct OCR1AL;

impl Register for OCR1AL {
    const ADDRESS: Address = 0x88 as Address;
}
