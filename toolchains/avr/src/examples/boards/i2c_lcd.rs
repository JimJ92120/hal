use lib_boards::arduino_uno::{ I2C, I2CSettings, I2CBitRate };

use crate::helpers;

pub fn run() {
    const I2C_SETTINGS: I2CSettings = I2CSettings {
        bit_rate: I2CBitRate::SixtyFour,
        slave_address: 0x27 << 1,
        baud_rate: 72,
    };
    const HIGH_BIT_MASK: u8 = 0b0000_1100;
    const LOW_BIT_MASK: u8 = 0b0000_1000;

    I2C::init(I2C_SETTINGS);
    helpers::delay(100);

    // set to 4-bit
    I2C::send(I2C_SETTINGS.slave_address, 0x30, HIGH_BIT_MASK, LOW_BIT_MASK);
    helpers::delay(100);
    I2C::send(I2C_SETTINGS.slave_address, 0x30, HIGH_BIT_MASK, LOW_BIT_MASK);
    helpers::delay(100);
    I2C::send(I2C_SETTINGS.slave_address, 0x30, HIGH_BIT_MASK, LOW_BIT_MASK);
    helpers::delay(100);
    I2C::send(I2C_SETTINGS.slave_address, 0x20, HIGH_BIT_MASK, LOW_BIT_MASK);
    helpers::delay(100);

    // clear
    I2C::send(I2C_SETTINGS.slave_address, 0x01, HIGH_BIT_MASK, LOW_BIT_MASK);
    helpers::delay(100);

    loop {}
}
