// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct PCIFR;

impl Register<u8> for PCIFR {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x1B) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum PCIFRBitField {
    PCIF0 = 0,
    PCIF1 = 1,
    PCIF2 = 2,
    // 3
    // 4
    // 5
    // 6
    // 7
}
