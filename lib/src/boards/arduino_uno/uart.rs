use crate::{
    globals::{ Register },
    registers::atmega328p::{
        UBRR0L,
        UBRR0H,
        UCSR0A, UCSR0ABitField,
        UCSR0C, UCSR0CBitField,
        UCSR0B, UCSR0BBitField,
        UDR0,
    }
};
pub struct UART;

impl UART {
    pub fn init(baud_rate: u32, frequency: u32, enable_transmission: bool, enable_reception: bool) {
        // set baud rate
        UBRR0H::clear();
        UBRR0L::set(Self::calculate_baud_rate_from_frequency(frequency, baud_rate));

        // set data frame format to 8 bits + 1 stop bit
        UCSR0C::set_bit_mask(UCSR0CBitField::UCSZ00 as u8);
        UCSR0C::set_bit_mask(UCSR0CBitField::UCSZ01 as u8);

        if enable_transmission {
            UCSR0B::set_bit_mask(UCSR0BBitField::TXEN0 as u8);
        }

        if enable_reception {
            UCSR0B::set_bit_mask(UCSR0BBitField::RXEN0 as u8);
        }
    }

    pub fn send(content: &str) {
        for byte in content.as_bytes() {
            while !(UCSR0A::is_bit_mask_set(UCSR0ABitField::UDRE0 as u8)) {}
            
            UDR0::set(*byte);

        }
    }

    pub fn read() -> Option<u8> {
        if UCSR0A::is_bit_mask_set(UCSR0ABitField::RXC0 as u8) {
            return Some(UDR0::get());
        }

        None
    }

    fn calculate_baud_rate_from_frequency(frequency: u32, baud_rate: u32) -> u8 {
        ((frequency / (frequency / 1_000_000) / baud_rate) - 1) as u8
    }
}
