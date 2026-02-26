// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=23
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct GPIOR1;

impl Register<u8> for GPIOR1 {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x2A) as *mut u8;
}
