// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=13
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPL;

impl Register<u8> for SPL {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x3D) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum SPLBitField {
    SP0 = 0,
    SP1 = 1,
    SP2 = 2,
    SP3 = 3,
    SP4 = 4,
    SP5 = 5,
    SP6 = 6,
    SP7 = 7,
}
