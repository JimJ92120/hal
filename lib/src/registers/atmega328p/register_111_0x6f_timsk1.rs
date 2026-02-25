// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=112
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct TIMSK1;

impl Register for TIMSK1 {
    const ADDRESS: Address = 0x6F as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TIMSK1BitField {
    TOIE1 = 0,
    OCIE1A = 1,
    OCIE1B = 2,
    // 3
    // 4
    ICIE1 = 5,
    // 6
    // 7
}
