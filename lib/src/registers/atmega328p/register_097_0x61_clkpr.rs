// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=33
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct CLKPR;

impl Register for CLKPR {
    const ADDRESS: Address = 0x61 as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum CLKPRBitField {
    CLKPS0 = 0,
    CLKPS1 = 1,
    CLKPS2 = 2,
    CLKPS3 = 3,
    // 4
    // 5
    // 6
    CLKPCE = 7,
}
