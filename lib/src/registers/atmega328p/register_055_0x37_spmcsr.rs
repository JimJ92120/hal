// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#[repr(u8)]age=239
use crate::globals::Register;
use super::IO_OFFSET;

#[derive(Debug)]
pub struct SPMCSR;

impl Register<u8> for SPMCSR {
    const ADDRESS: *mut u8 = (IO_OFFSET + 0x37) as *mut u8;
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
