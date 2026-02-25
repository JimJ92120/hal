// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=160
use crate::globals::{ Register, Address };

#[derive(Debug)]
pub struct UCSR0B;

impl Register for UCSR0B {
    const ADDRESS: Address = 0xC1 as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum UCSR0BBitField {
    TXB80 = 0,
    RXB80 = 1,
    UCSZ02 = 2,
    TXEN0 = 3,
    RXEN0 = 4,
    UDRIE0 = 5,
    TXCIE0 = 6,
    RXCIE0 = 7,
}
