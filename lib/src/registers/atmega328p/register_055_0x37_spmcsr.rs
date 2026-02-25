// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=239
use crate::globals::{ Register, Address };
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPMCSR;

impl Register for SPMCSR {
    const ADDRESS: Address = (IO_OFFSET + 0x37) as Address;
}

#[derive(Debug)]
#[repr(u8)]
pub enum SPMCSRBitField {
    SELFPRGN = 0,
    PGERS = 1,
    PGWRT = 2,
    BLBSET = 3,
    RWWSRE = 4,
    // 5
    RWWSB = 6,
    SPMIE = 7,
}
