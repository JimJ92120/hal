use format_no_std::show;

use lib_boards::arduino_uno::{
    UART, UARTSettings, UARTCharSize, UARTStopBit, UARTSyncMode, UARTParityMode,
};

use crate::helpers;

pub fn run() {
    const DELAY_DURATION: u32 = 1_000_000;
    
    UART::init(UARTSettings {
        baud_rate: 57_600,
        frequency: 16_000_000,
        enable_reception: false,
        enable_transmission: true,
        char_size: UARTCharSize::Eight,
        stop_bit: UARTStopBit::One,
        sync_mode: UARTSyncMode::Async,
        parity_mode: UARTParityMode::Disabled,
    });

    let mut buffer: [u8; 64] = [0; 64];

    loop {
        UART::send(show(
            &mut buffer, format_args!("hello {}\n", "world")
        ).unwrap());
        helpers::delay(DELAY_DURATION);
    }
}
