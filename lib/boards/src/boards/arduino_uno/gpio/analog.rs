use lib_registers::{
    globals::Register,
    atmega328p::{
        ADMUX, ADMUXBitField,
        ADCSRA, ADCSRABitField,
        ADCL, ADCH,
    }
};

use super::Pin;

#[derive(Debug)]
pub struct Analog;

impl Analog {
    pub fn init(pin: Pin) {
        match pin {
            Pin::Fourteen => (),
            Pin::Fifteen => ADMUX::or(1 << ADMUXBitField::MUX3 as u8),
            Pin::Sixteen => ADMUX::or(1 << ADMUXBitField::MUX2 as u8),
            Pin::Seventeen => ADMUX::or(1 << ADMUXBitField::MUX1 as u8),
            Pin::Eighteen => ADMUX::or(1 << ADMUXBitField::MUX0 as u8),

            _ => panic!("Pin doesn't support analog."),
        };

        ADMUX::or(1 << ADMUXBitField::REFS1 as u8);
        ADMUX::or(1 << ADMUXBitField::REFS0 as u8);
        ADCSRA::or(1 << ADCSRABitField::ADPS0 as u8);
        ADCSRA::or(1 << ADCSRABitField::ADPS1 as u8);
        ADCSRA::or(1 << ADCSRABitField::ADPS2 as u8);
        ADCSRA::or(1 << ADCSRABitField::ADEN as u8);
    }

    pub fn read() -> u16 {
        ADCSRA::or(1 << ADCSRABitField::ADSC as u8);

        while 0 != ADCSRA::get() & (1 << ADCSRABitField::ADSC as u8) {}

        (ADCL::get() as u16) | ((ADCH::get() as u16) << 8)
    }
}
