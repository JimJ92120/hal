#![no_std]
#![no_main]

use hal_lib::{
    globals::{ Register },
    registers::bcm2837::{
        GPFSEL2, GPFSEL2BitField,
        GPSET0, GPSET0BitField,
        GPCLR0, GPCLR0BitField
    },
};

mod panic;
mod boot;
mod helpers;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    const DELAY_DURATION: u32 = 3_000_000;
    
    // set GPIO 27 as output
    GPFSEL2::set_bit_mask(GPFSEL2BitField::FSEL7 as u32);

    loop {
        // set high
        GPSET0::set_bit_mask(GPSET0BitField::PIN27 as u32);

        helpers::delay(DELAY_DURATION);

        // set low
        GPCLR0::set_bit_mask(GPCLR0BitField::PIN27 as u32);

        helpers::delay(DELAY_DURATION);
    }
}
