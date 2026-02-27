// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=13
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPH;

impl Register<u8> for SPH {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x3E) as *mut u8;
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
