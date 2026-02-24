#![no_std]
#![no_main]

use hal_lib::{
    globals::{ Register },
    registers::bcm2837::{ GPFSEL2, GPSET0, GPCLR0 },
};

mod panic;
mod boot;
mod helpers;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    const DELAY_DURATION: u32 = 3_000_000;
    
    unsafe {
        // set GPIO 27 as output
        GPFSEL2::set(GPFSEL2::FSEL7.mask());
    }

    loop {
        unsafe {
            // set high
            GPSET0::set(GPSET0::PIN_27.mask());
        }

        helpers::delay(DELAY_DURATION);

        unsafe {
            // set low
            GPCLR0::set(GPCLR0::PIN_27.mask());
        }

        helpers::delay(DELAY_DURATION);
    }
}
