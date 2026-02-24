// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=111
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TCNT1L;

impl Register for TCNT1L {
    const ADDRESS: Address = 0x84 as Address;
}

impl TCNT1L {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
