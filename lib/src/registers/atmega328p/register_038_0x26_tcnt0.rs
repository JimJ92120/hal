// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct TCNT0;

impl Register<u8> for TCNT0 {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x26) as *mut u8;
}
