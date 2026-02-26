// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=111
use crate::globals::Register;

#[derive(Debug)]
pub struct OCR1AH;

impl Register<u8> for OCR1AH {
    const ADDRESS: *mut u8 = 0x89 as *mut u8;
}
