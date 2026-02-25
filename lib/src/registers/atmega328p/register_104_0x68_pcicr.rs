// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct PCICR;

impl Register for PCICR {
    const ADDRESS: Address = 0x68 as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum PCICRBitField {
    PCIE0 = 0,
    PCIE1 = 1,
    PCIE2 = 2,
    // 3
    // 4
    // 5
    // 6
    // 7
}
