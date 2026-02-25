// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=112
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct ICR1H;

impl Register for ICR1H {
    const ADDRESS: Address = 0x87 as Address;
}
