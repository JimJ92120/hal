// see https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=184
use lib_registers::{
    globals::{ Register },
    atmega328p::{
        TWCR, TWCRBitField,
        TWSR, TWSRBitField,
        TWDR,
        TWAR,
        TWBR,
    }
};

mod settings;

pub use settings::{ I2CSettings, I2CBitRate };

#[derive(Debug)]
pub struct I2C;

impl I2C {
    pub fn init(settings: I2CSettings) {
        Self::stop_transmission();

        Self::set_bit_rate(settings.bit_rate);

        TWBR::set(settings.baud_rate);
        TWCR::set(
            (1 << TWCRBitField::TWEN as u8)
            | (1 << TWCRBitField::TWIE as u8)
            | (1 << TWCRBitField::TWEA as u8)
        );
        TWAR::set(settings.slave_address);
    }

    pub fn send(address: u8, data: u8, high_bit_mask: u8, low_bit_mask: u8) {
        Self::start_transmission();
        Self::send_byte(address);

        for byte in Self::parse_data(data, high_bit_mask, low_bit_mask) {
            Self::send_byte(byte);
        }

        Self::stop_transmission();
    }

    fn start_transmission() {
        let bit_mask: u8 = (1 << TWCRBitField::TWINT as u8)
            | (1 << TWCRBitField::TWEA as u8)
            | (1 << TWCRBitField::TWEN as u8)
            | (1 << TWCRBitField::TWIE as u8);

        // repeated start
        if Self::is_repeated_start() {
            TWCR::set(bit_mask);

            while 0 != (TWCR::get() & (1 << TWCRBitField::TWWC as u8)) {}
        } else {
            TWCR::set(
                bit_mask
                | (1 << TWCRBitField::TWSTA as u8)
            );
        }

        while 0 == (TWCR::get() & (1 << TWCRBitField::TWINT as u8)) {}
    }

    fn stop_transmission() {
        TWCR::set(
            (1 << TWCRBitField::TWINT as u8)
            | (1 << TWCRBitField::TWEA as u8)
            | (1 << TWCRBitField::TWEN as u8)
            | (1 << TWCRBitField::TWIE as u8)
            | (1 << TWCRBitField::TWSTO as u8)
        );

        while 0 != (TWCR::get() & (1 << TWCRBitField::TWINT as u8)) {}
    }

    fn send_byte(data: u8) {
        TWDR::set(data);
        TWCR::set(
            (1 << TWCRBitField::TWINT as u8)
            | (1 << TWCRBitField::TWEA as u8)
            | (1 << TWCRBitField::TWEN as u8)
            | (1 << TWCRBitField::TWIE as u8)
        );

        while 0 == (TWCR::get() & (1 << TWCRBitField::TWINT as u8)) {}
    }

    fn parse_data(data: u8, high_bit_mask: u8, low_bit_mask: u8) -> [u8; 4] {
        let high_bits = data & 0b1111_0000;
        let low_bits = (data & 0b0000_1111) << 4;
        
        [
            high_bits | high_bit_mask,
            high_bits | low_bit_mask,
            low_bits | high_bit_mask,
            low_bits | low_bit_mask,
        ]
    }

    fn set_bit_rate(bit_rate: I2CBitRate) {
        match bit_rate {
            I2CBitRate::One       => (),
            I2CBitRate::Four      => TWSR::or(1 << TWSRBitField::TWPS0 as u8),
            I2CBitRate::Sixteen   => TWSR::or(1 << TWSRBitField::TWPS1 as u8),
            I2CBitRate::SixtyFour => {
                TWSR::or(1 << TWSRBitField::TWPS0 as u8);
                TWSR::or(1 << TWSRBitField::TWPS1 as u8);
            },
        };
    }

    fn is_repeated_start() -> bool {
        10 == TWSR::get()
    }
}
