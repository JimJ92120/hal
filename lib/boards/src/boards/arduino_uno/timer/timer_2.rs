use lib_registers::{
    globals::{ Register },
    atmega328p::{
        TCCR2A, TCCR2ABitField,
        TCCR2B, TCCR2BBitField,
        OCR2A, OCR2B,
    }
};

use super::Timer;

pub struct Timer2;

impl Timer<u8> for Timer2 {
    fn init() {
        TCCR2A::set(0);
        TCCR2B::set(0);

        TCCR2A::or(1 << TCCR2ABitField::WGM21 as u8);
        TCCR2A::or(1 << TCCR2ABitField::WGM20 as u8);
        TCCR2A::or(1 << TCCR2ABitField::COM2A1 as u8);
        TCCR2A::or(1 << TCCR2ABitField::COM2B1 as u8);

        TCCR2B::or(1 << TCCR2BBitField::CS22 as u8);
    }

    // GPIO 11
    fn set_a(value: u8) {
        OCR2A::set(value);
    }

    // GPIO 3
    fn set_b(value: u8) {
        OCR2B::set(value);
    }
}
