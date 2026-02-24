#![no_std]
#![no_main]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

use hal_lib::{
    globals::{ Register, Bit },
    registers::atmega328p::{ PORTB, DDRB },
};

mod panic;
mod helpers;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    const DELAY_DURATION: u32 = 1_000_000;
    // GPIO 13 / LED_BUILTIN
    const PIN: Bit = DDRB::PB5;

    unsafe {
        // set output
        DDRB::set(PIN.mask());
    }

    loop {
        unsafe {
            // high
            PORTB::set(PIN.mask());
        }

        helpers::delay(DELAY_DURATION);

        unsafe {
            // low
            PORTB::set(PORTB::get() & !(PIN.mask()));
        }


        helpers::delay(DELAY_DURATION);
    }
}
