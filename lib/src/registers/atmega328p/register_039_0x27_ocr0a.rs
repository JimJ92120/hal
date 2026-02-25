// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct OCR0A;

impl Register for OCR0A {
    const ADDRESS: Address = (IO_OFFSET + 0x27) as Address;
}
