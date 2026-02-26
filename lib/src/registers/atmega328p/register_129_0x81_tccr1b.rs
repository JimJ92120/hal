// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=110
use crate::globals::Register;

#[derive(Debug)]
pub struct TCCR1B;

impl Register<u8> for TCCR1B {
    const ADDRESS: *mut u8 = 0x81 as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum TCCR1BBitField {
    CS10 = 0,
    CS11 = 1,
    CS12 = 2,
    WGM12 = 3,
    WGM13 = 4,
    // 5
    ICES1 = 6,
    ICNC1 = 7,
}
