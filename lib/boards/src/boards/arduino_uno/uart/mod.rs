use lib_registers::{
    globals::{ Register },
    atmega328p::{
        UBRR0L,
        UBRR0H,
        UCSR0A, UCSR0ABitField,
        UCSR0C, UCSR0CBitField,
        UCSR0B, UCSR0BBitField,
        UDR0,
    }
};

mod settings;

pub use settings::{ UARTSettings, UARTCharSize, UARTStopBit, UARTSyncMode, UARTParityMode };

#[derive(Debug)]
pub struct UART;

impl UART {
    pub fn init(settings: UARTSettings) {
        let UARTSettings {
            frequency,
            baud_rate,
            enable_reception,
            enable_transmission,
            char_size,
            stop_bit,
            sync_mode,
            parity_mode,
        } = settings;

        Self::set_baud_rate(baud_rate, frequency);
        Self::set_char_size(char_size);
        Self::set_stop_bit(stop_bit);
        Self::set_sync_mode(sync_mode);
        Self::set_parity_mode(parity_mode);

        if enable_transmission {
            UCSR0B::or(1 << UCSR0BBitField::TXEN0 as u8);
        }

        if enable_reception {
            UCSR0B::or(1 << UCSR0BBitField::RXEN0 as u8);
        }
    }

    pub fn send(content: &str) {
        for byte in content.as_bytes() {
            while !(0 != UCSR0A::get() & (1 << UCSR0ABitField::UDRE0 as u8)) {}
            
            UDR0::set(*byte);

        }
    }

    pub fn read() -> Option<u8> {
        if 0 != UCSR0A::get() & (1 << UCSR0ABitField::RXC0 as u8) {
            return Some(UDR0::get());
        }

        None
    }

    fn set_baud_rate(baud_rate: u32, frequency: u32) {
        let baud_rate = ((frequency / (frequency / 1_000_000) / baud_rate) - 1) as u8;

        UBRR0H::set(0);
        UBRR0L::set(baud_rate);
    }

    // only 8-bit supported in `UART::send()`
    fn set_char_size(mode: UARTCharSize) {
        match mode {
            UARTCharSize::Five => (),
            UARTCharSize::Six => UCSR0C::or(1 << UCSR0CBitField::UCSZ00 as u8),
            UARTCharSize::Seven => UCSR0C::or(1 << UCSR0CBitField::UCSZ01 as u8),
            UARTCharSize::Eight => {
                UCSR0C::or(1 << UCSR0CBitField::UCSZ00 as u8);
                UCSR0C::or(1 << UCSR0CBitField::UCSZ01 as u8);
            },
            UARTCharSize::Nine => {
                UCSR0C::or(1 << UCSR0CBitField::UCSZ00 as u8);
                UCSR0C::or(1 << UCSR0CBitField::UCSZ01 as u8);
                UCSR0B::or(1 << UCSR0BBitField::UCSZ02 as u8);
            },
        };
    }

    fn set_stop_bit(stop_bit: UARTStopBit) {
        match stop_bit {
            UARTStopBit::One => (),
            UARTStopBit::Two => UCSR0C::or(1 << UCSR0CBitField:: USBS0 as u8),
        };
    }

    fn set_sync_mode(mode: UARTSyncMode) {
        match mode {
            UARTSyncMode::Async => (),
            UARTSyncMode::Sync => UCSR0C::or(1 << UCSR0CBitField::UMSEL00 as u8),
            UARTSyncMode::SPI => {
                UCSR0C::or(1 << UCSR0CBitField::UMSEL00 as u8);
                UCSR0C::or(1 << UCSR0CBitField::UMSEL01 as u8);
            }
        };
    }

    fn set_parity_mode(mode: UARTParityMode) {
        match mode {
            UARTParityMode::Disabled => (),
            UARTParityMode::Even => UCSR0C::or(1 << UCSR0CBitField::UPM00 as u8),
            UARTParityMode::Odd => {
                UCSR0C::or(1 << UCSR0CBitField::UPM00 as u8);
                UCSR0C::or(1 << UCSR0CBitField::UPM01 as u8);
            },
        };
    }
}
