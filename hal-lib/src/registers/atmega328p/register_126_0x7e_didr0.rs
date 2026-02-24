// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=220
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct DIDR0;

impl Register for DIDR0 {
    const ADDRESS: Address = 0x7E as Address;
}

impl DIDR0 {
    pub const ADC0D: Bit = Bit::Zero;
    pub const ADC1D: Bit = Bit::One;
    pub const ADC2D: Bit = Bit::Two;
    pub const ADC3D: Bit = Bit::Three;
    pub const ADC4D: Bit = Bit::Four;
    pub const ADC5D: Bit = Bit::Five;
    // 6
    // 7
}
