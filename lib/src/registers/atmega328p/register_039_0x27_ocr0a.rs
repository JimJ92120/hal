// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct OCR0A;

impl Register<u8> for OCR0A {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x27) as *mut u8;
}
