// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=220
use crate::globals::Register;

#[derive(Debug)]
pub struct DIDR0;

impl Register<u8> for DIDR0 {
    const ADDRESS: *mut u8 = 0x7E as *mut u8;
}

#[derive(Debug)]
#[repr(u8)]
pub enum DIDR0BitField {
    ADC0D = 0,
    ADC1D = 1,
    ADC2D = 2,
    ADC3D = 3,
    ADC4D = 4,
    ADC5D = 5,
    // 6
    // 7
}
