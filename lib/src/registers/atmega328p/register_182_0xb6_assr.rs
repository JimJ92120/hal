// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=133
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct ASSR;

impl Register for ASSR {
    const ADDRESS: Address = 0xB6 as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum ASSRBitField {
    TCR2BUB = 0,
    TCR2AUB = 1,
    OCR2BUB = 2,
    OCR2AUB = 3,
    TCN2UB = 4,
    AS2 = 5,
    EXCLK = 6,
    // 7
}
