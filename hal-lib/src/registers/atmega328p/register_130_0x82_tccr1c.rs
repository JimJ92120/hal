// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TCCR1C;

impl Register for TCCR1C {
    const ADDRESS: Address = 0x82 as Address;
}

impl TCCR1C {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    pub const FOC1B: Bit = Bit::Six;
    pub const FOC1A: Bit = Bit::Seven;
}
