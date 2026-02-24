#![allow(dead_code)]

use hal_lib::{
    globals::{ Register as RegisterTrait },
    registers::atmega328p::PORTB as Register
};

fn main() {
    println!("register: {:?}", Register {});
    println!("address: {:?}", Register::ADDRESS);
    println!("bit value: {:x}", Register::PB2 as u8);
    println!("bit mask: {:08b}", Register::PB2.mask());
}
