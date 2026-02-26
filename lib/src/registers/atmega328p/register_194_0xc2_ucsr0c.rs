// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=161
// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=172
use crate::globals::Register;

#[derive(Debug)]
pub struct UCSR0C;

impl Register<u8> for UCSR0C {
    const ADDRESS: *mut u8 = 0xC2 as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum UCSR0CBitField {
    UCPOL0 = 0,
    UCSZ00 = 1,
    UCSZ01 = 2,
    USBS0 = 3,
    UPM00 = 4,
    UPM01 = 5,
    UMSEL00 = 6,
    UMSEL01 = 7,

    // alternative naming
    // UCPHA0 = 1,
    // UDORD0 = 2,
}
