// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=200
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct TWSR;

impl Register for TWSR {
    const ADDRESS: Address = 0xB9 as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TWSRBitField {
    TWPS0 = 0,
    TWPS1 = 1,
    // 2
    TWS3 = 3,
    TWS4 = 4,
    TWS5 = 5,
    TWS6 = 6,
    TWS7 = 7,
}
