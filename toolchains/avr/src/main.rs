#![no_std]
#![no_main]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use hal_lib::{
    boards::arduino_uno::{ Pin, GPIO }
};

mod panic;
mod helpers;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    const DELAY_DURATION: u32 = 1_000_000;
    // GPIO 13 / LED_BUILTIN
    const PIN: Pin = Pin::Thirteen;
    
    GPIO::set_output(PIN);

    loop {
        GPIO::set_high(PIN);
        helpers::delay(DELAY_DURATION);

        GPIO::set_low(PIN);
        helpers::delay(DELAY_DURATION);
    }
}
