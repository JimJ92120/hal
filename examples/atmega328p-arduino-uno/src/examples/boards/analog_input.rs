use format_no_std::show;

use lib_boards::arduino_uno::{
    Pin,
    Analog, AnalogSettings, AnalogMode, AnalogPrescaler,
    UART, UARTSettings, UARTCharSize, UARTStopBit, UARTSyncMode, UARTParityMode,
};

use crate::helpers;

pub fn run() {
    const DELAY_DURATION: u32 = 500_000;
    // GPIO 19
    const PIN: Pin = Pin::Nineteen;
    const SETTINGS: AnalogSettings = AnalogSettings {
        mode: AnalogMode::AVcc,
        prescaler: AnalogPrescaler::HundredTwentyEight,
    };
    
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
    Analog::init();

    let mut buffer: [u8; 64] = [0; 64];

    loop {
        Analog::start_conversion(PIN, SETTINGS);

        UART::send(show(
            &mut buffer, format_args!("10-bit: {}\n", Analog::read())
        ).unwrap());

        helpers::delay(DELAY_DURATION);
    }
}
