use lib_registers::{
    globals::{ Register },
    atmega328p::{
        TCCR1A, TCCR1ABitField,
        TCCR1B, TCCR1BBitField,
        OCR1AL, OCR1AH, OCR1BL, OCR1BH,
        ICR1L, ICR1H,
    }
};

use super::Timer;

pub struct Timer1;

impl Timer<u16> for Timer1 {
    fn init() {
        TCCR1A::set(0);
        TCCR1B::set(0);

        TCCR1A::or(1 << TCCR1ABitField::WGM11 as u8);
        TCCR1B::or(1 << TCCR1BBitField::WGM12 as u8);
        TCCR1B::or(1 << TCCR1BBitField::WGM13 as u8);
        TCCR1A::or(1 << TCCR1ABitField::COM1A1 as u8);
        TCCR1A::or(1 << TCCR1ABitField::COM1B1 as u8);

        ICR1L::set(0b1111_1111);
        ICR1H::set(0b1111_1111);

        TCCR1B::or(1 << TCCR1BBitField::CS10 as u8);
    }

    // GPIO 9
    fn set_a(value: u16) {
        OCR1AL::set((value >> 8).try_into().unwrap());
        OCR1AH::set((value & 0b1111_1111).try_into().unwrap());
    }

    // GPIO 10
    fn set_b(value: u16) {
        OCR1BL::set((value >> 8).try_into().unwrap());
        OCR1BH::set((value & 0b1111_1111).try_into().unwrap());
    }
}
