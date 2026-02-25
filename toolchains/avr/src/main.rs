#![no_std]
#![no_main]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use hal_lib::{
    boards::arduino_uno::{ Pin, GPIO, UART }
};

mod panic;
mod helpers;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    const DELAY_DURATION: u32 = 1_000_000;
    const FREQUENCY: u32 = 16_000_000;
    const BAUD_RATE: u32 = 57_600;
    const ENABLE_TRANSMISSION: bool = true;
    const ENABLE_RECEPTION: bool = true;
    // GPIO 13 / LED_BUILTIN
    const PIN: Pin = Pin::Thirteen;
    
    UART::init(BAUD_RATE, FREQUENCY, ENABLE_TRANSMISSION, ENABLE_RECEPTION);
    GPIO::set_output(PIN);

    loop {
        if let Some(byte) = UART::read() {
            UART::send("received: ");
            UART::send(core::str::from_utf8(&[byte]).unwrap());
            UART::send("\n");
        }

        UART::send("on\n");
        GPIO::set_high(PIN);
        helpers::delay(DELAY_DURATION);

        UART::send("off\n");
        GPIO::set_low(PIN);
        helpers::delay(DELAY_DURATION);
    }
}
