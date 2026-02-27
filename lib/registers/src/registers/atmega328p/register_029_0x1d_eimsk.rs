// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=55
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct EIMSK;

impl Register<u8> for EIMSK {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x1D) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum EIMSKBitField {
    INT0 = 0,
    INT1 = 1,
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
