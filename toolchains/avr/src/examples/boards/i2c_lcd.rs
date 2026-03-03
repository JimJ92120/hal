// see https://www.waveshare.com/w/upload/4/4d/LCD1602_I2C_Module.pdf?srsltid=AfmBOor4UWfAB5agz1WrT-w3aDwD-xntqWkoCSqTosnbvIu7V4gQE_qd
use lib_boards::arduino_uno::{ I2C, I2CSettings, I2CBitRate };

use crate::helpers;

struct Settings;

impl Settings {
    const CLEAR: u8             = 0b0000_0001;
    const RETURN_HOME: u8       = 0b0000_0010;

    const ENTRY_SET: u8         = 0b0000_0100;
    const INCREMENT_BY_1: u8    = 0b0000_0010; // 0 = no increment
    const SHIFT: u8             = 0b0000_0001; // 0 = no shift

    const DISPLAY_SET: u8       = 0b0000_1000;
    const DISPLAY: u8           = 0b0000_0100; // 0 = off
    const CURSOR: u8            = 0b0000_0010; // 0 = off
    const BLINK: u8             = 0b0000_0001; // 0 = off

    const CURSOR_SET: u8        = 0b0001_0100;
    const MOVE: u8              = 0b0000_1000; // 0 = shift
    const TO_LEFT: u8           = 0b0000_0100; // 0 = right

    const FUNCTION_SET: u8      = 0b0010_0000;
    const LINES_2: u8           = 0b0000_1000; // 0 = 1 line
    const FONT_5_10: u8         = 0b0000_0100; // 0 = 5x8 font

    const CGRAM_SET: u8         = 0b0100_0000;

    const DDRAM_SET: u8         = 0b1000_0000;
    const ROW_2: u8             = 0b0100_0000;
}

fn send(address: u8, data: u8, is_data: bool, delay: u32) {
    let mut bit_masks: [u8; 2] = [
        0b0000_1100,
        0b0000_1000,
    ];

    if is_data {
        bit_masks[0] |= 1;
        bit_masks[1] |= 1;
    }

    I2C::send(address, data, bit_masks[0], bit_masks[1]);
    helpers::delay(delay);
}

fn send_string(address: u8, content: &str, delay: u32) {
    for byte in content.as_bytes() {
        send(address, *byte, true, delay);
    }
}

// not checking display size
// set for 16x2 (2 rows / 16 columns)
fn move_cursor(address: u8, row_index: u8, column_index: u8, delay: u32) {
    let mut bit_mask: u8 = Settings::DDRAM_SET | column_index;

    if 1 == row_index {
        bit_mask |= Settings::ROW_2;
    }

    send(address, bit_mask, false, delay);
}

pub fn run() {
    const I2C_SETTINGS: I2CSettings = I2CSettings {
        bit_rate: I2CBitRate::One,
        slave_address: 0x27 << 1,
        baud_rate: 72,
    };

    I2C::init(I2C_SETTINGS);

    // wait possible LCD to be up
    helpers::delay(500);

    // set to 4-bit
    send(I2C_SETTINGS.slave_address, 0b0011_0000, false, 500); // 0x30
    send(I2C_SETTINGS.slave_address, 0b0011_0000, false, 500);
    send(I2C_SETTINGS.slave_address, 0b0011_0000, false, 500);
    send(I2C_SETTINGS.slave_address, 0b0010_0000, false, 500); // 0x20

    // settings
    send(
        I2C_SETTINGS.slave_address,
        Settings::FUNCTION_SET | Settings::LINES_2,
        false,
        500
    );
    send(
        I2C_SETTINGS.slave_address,
        Settings::CLEAR,
        false,
        500
    );
    send(
        I2C_SETTINGS.slave_address,
        Settings::DISPLAY_SET,
        false,
        500
    );
    send(
        I2C_SETTINGS.slave_address,
        Settings::ENTRY_SET | Settings::INCREMENT_BY_1,
        false,
        500
    );
    send(
        I2C_SETTINGS.slave_address,
        Settings::DISPLAY_SET | Settings::DISPLAY | Settings::CURSOR | Settings::BLINK,
        false,
        500
    );
    send(
        I2C_SETTINGS.slave_address,
        Settings::RETURN_HOME,
        false,
        500
    );

    helpers::delay(1_000_000);
    
    send_string(
        I2C_SETTINGS.slave_address,
        "hello world",
        500_000,
    );
    move_cursor(
        I2C_SETTINGS.slave_address,
        1,
        2,
        500
    );
    send_string(
        I2C_SETTINGS.slave_address,
        "hallo welt",
        500_000,
    );

    loop {}
}
