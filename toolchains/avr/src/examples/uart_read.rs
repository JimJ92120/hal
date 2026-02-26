use hal_lib::{
    boards::arduino_uno::UART
};

pub fn run() {
    const DELAY_DURATION: u32 = 1_000_000;
    const FREQUENCY: u32 = 16_000_000;
    const BAUD_RATE: u32 = 57_600;
    const ENABLE_TRANSMISSION: bool = true;
    const ENABLE_RECEPTION: bool = true;
    
    UART::init(BAUD_RATE, FREQUENCY, ENABLE_TRANSMISSION, ENABLE_RECEPTION);

    loop {
        if let Some(byte) = UART::read() {
            UART::send("received: ");
            UART::send(core::str::from_utf8(&[byte]).unwrap());
            UART::send("\n");
        }
    }
}
