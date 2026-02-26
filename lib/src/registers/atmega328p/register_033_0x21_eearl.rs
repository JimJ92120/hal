// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=20
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct EEARL;

impl Register<u8> for EEARL {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x21) as *mut u8;
}
