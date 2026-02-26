// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=142
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPDR;

impl Register<u8> for SPDR {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x2E) as *mut u8;
}
