// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=141
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPSR;

impl Register<u8> for SPSR {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x2D) as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum SPSRBitField {
    SPI2X = 0,
    // 1
    // 2
    // 3
    // 4
    // 5
    WCOL = 6,
    SPIF = 7,
}
