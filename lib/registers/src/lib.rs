#![no_std]
#![no_main]

#![allow(dead_code)]
#![allow(unused_imports)]
#[allow(non_camel_case_types)]

mod registers;

pub mod globals;

#[cfg(feature = "atmega328p")]
pub use registers::atmega328p;

// see registers/bcm/README.md
#[cfg(any(
    feature = "bcm2835",
    feature = "bcm2836",
    feature = "bcm2837",
    feature = "bcm2837b0",
    feature = "bcm2711",
))]
pub use registers::bcm;
