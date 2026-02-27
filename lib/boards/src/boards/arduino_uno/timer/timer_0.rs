use lib_registers::{
    globals::{ Register },
    atmega328p::{
        TCCR0A, TCCR0ABitField,
        TCCR0B, TCCR0BBitField,
        OCR0A, OCR0B,
    }
};

use super::Timer;

pub struct Timer0;

impl Timer<u8> for Timer0 {
    fn init() {
        TCCR0A::set(0);
        TCCR0B::set(0);

        TCCR0A::or(1 << TCCR0ABitField::WGM01 as u8);
        TCCR0A::or(1 << TCCR0ABitField::WGM00 as u8);
        TCCR0A::or(1 << TCCR0ABitField::COM0A1 as u8);
        TCCR0A::or(1 << TCCR0ABitField::COM0B1 as u8);

        TCCR0B::or(1 << TCCR0BBitField::CS02 as u8);
    }

    // GPIO 6
    fn set_a(value: u8) {
        OCR0A::set(value);
    }

    // GPIO 5
    fn set_b(value: u8) {
        OCR0B::set(value);
    }
}
