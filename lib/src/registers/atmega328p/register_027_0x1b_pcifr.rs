// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct PCIFR;

impl Register for PCIFR {
    const ADDRESS: Address = (IO_OFFSET + 0x1B) as Address;
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
