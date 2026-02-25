// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=132
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct TIMSK2;

impl Register for TIMSK2 {
    const ADDRESS: Address = 0x70 as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TIMSK2BitField {
    TOIE2 = 0,
    OCIE2A = 1,
    OCIE2B = 2,
    // 3
    // 4
    // 5
    // 6
    // 7
}
