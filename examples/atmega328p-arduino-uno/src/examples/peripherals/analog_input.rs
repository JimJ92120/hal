use format_no_std::show;

use lib_peripherals::arduino_uno::{
    Pin,
    Potentiometer,
    AnalogSettings, AnalogMode, AnalogPrescaler,
};
use lib_boards::arduino_uno::{
    UART, UARTSettings, UARTCharSize, UARTStopBit, UARTSyncMode, UARTParityMode,
};

use crate::helpers;

pub fn run() {
    const DELAY_DURATION: u32 = 500_000;
    
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

    let potentiometer = Potentiometer::new(
        Pin::Nineteen,
        AnalogSettings {
            mode: AnalogMode::AVcc,
            prescaler: AnalogPrescaler::HundredTwentyEight,
        }
    );

    loop {
        UART::send(show(
            &mut buffer, format_args!("10-bit: {}\n", potentiometer.value())
        ).unwrap());

        helpers::delay(DELAY_DURATION);
    }
}
