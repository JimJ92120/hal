use format_no_std::show;

use lib_boards::arduino_uno::UART;

use crate::helpers;

pub fn run() {
    const DELAY_DURATION: u32 = 1_000_000;
    const FREQUENCY: u32 = 16_000_000;
    const BAUD_RATE: u32 = 57_600;
    const ENABLE_TRANSMISSION: bool = true;
    const ENABLE_RECEPTION: bool = true;
    
    UART::init(BAUD_RATE, FREQUENCY, ENABLE_TRANSMISSION, ENABLE_RECEPTION);

    let mut buffer: [u8; 64] = [0; 64];

    loop {
        UART::send(show(
            &mut buffer, format_args!("hello {}\n", "world")
        ).unwrap());
        helpers::delay(DELAY_DURATION);
    }
}
