// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=131
use crate::globals::{ Register, Bit, Address };

#[derive(Debug)]
pub struct TCNT2;

impl Register for TCNT2 {
    const ADDRESS: Address = 0xB2 as Address;
}

impl TCNT2 {
    // 0
    // 1
    // 2
    // 3
    // 4
    // 5
    // 6
    // 7
}
