#![no_std]
#![no_main]
#![allow(dead_code)]

#![feature(asm_experimental_arch)]
#[cfg(target_arch = "avr")]

mod panic;
mod helpers;

mod examples;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    examples::led_blink::run();
    // examples::button_toggle::run();
    // examples::uart_send::run();
    // examples::uart_read::run();
    // examples::rgb_led::run();
}
