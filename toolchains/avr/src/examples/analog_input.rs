use format_no_std::show;

use lib_boards::arduino_uno::{ GPIO, Pin, Analog, UART };

use crate::helpers;

pub fn run() {
    const DELAY_DURATION: u32 = 500_000;
    const FREQUENCY: u32 = 16_000_000;
    const BAUD_RATE: u32 = 57_600;
    const ENABLE_TRANSMISSION: bool = true;
    const ENABLE_RECEPTION: bool = true;
    // GPIO 14
    const PIN: Pin = Pin::Fourteen;
    
    GPIO::set_input(PIN);
    UART::init(BAUD_RATE, FREQUENCY, ENABLE_TRANSMISSION, ENABLE_RECEPTION);
    Analog::init(PIN);

    let mut buffer: [u8; 64] = [0; 64];

    loop {
        UART::send(show(
            &mut buffer, format_args!("10-bit: {}\n", Analog::read())
        ).unwrap());

        buffer = [0; 64];

        helpers::delay(DELAY_DURATION);
    }
}
