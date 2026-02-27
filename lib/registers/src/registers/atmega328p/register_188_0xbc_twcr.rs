// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=199
use crate::globals::Register;

#[derive(Debug)]
pub struct TWCR;

impl Register<u8> for TWCR {
    const ADDRESS: *mut u8 = 0xBC as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TWCRBitField {
    TWIE = 0,
    // 2
    TWEN = 2,
    TWWC = 3,
    TWSTO = 4,
    TWSTA = 5,
    TWEA = 6,
    TWINT = 7,
}
