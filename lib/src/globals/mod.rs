mod register_8_bits;
mod register_32_bits;

#[cfg(feature = "atmega328p")]
pub use register_8_bits::{ Address, Register };

#[cfg(feature = "bcm2837")]
pub use register_32_bits::{ Address, Register };
