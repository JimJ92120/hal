// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=13
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPH;

impl Register for SPH {
    const ADDRESS: Address = (IO_OFFSET + 0x3E) as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum SPHBitField {
    SP8 = 0,
    SP9 = 1,
    SP10 = 2,
    // 3
    // 4
    // 5
    // 6
    // 7
}
