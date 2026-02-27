// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=88
use crate::globals::Register;

#[derive(Debug)]
pub struct TIMSK0;

impl Register<u8> for TIMSK0 {
    const ADDRESS: *mut u8 = 0x6E as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TIMSK0BitField {
    TOIE0 = 0,
    OCIE0A = 1,
    OCIE0B = 2,
    // 3
    // 4
    // 5
    // 6
    // 7
}
