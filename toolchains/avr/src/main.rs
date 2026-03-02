#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

mod panic;
mod helpers;

mod examples;
use examples::{ boards, peripherals };

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    // lib_boards
    // boards::led_blink::run();
    // boards::button_toggle::run();
    // boards::uart_send::run();
    // boards::uart_read::run();
    // boards::rgb_led::run();
    // boards::analog_input::run();
    // boards::i2c_lcd::run();

    // lib_peripherals
    peripherals::led_blink::run();
}
