// https://content.arduino.cc/assets/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf#page=72
use crate::globals::Register;
use super::MMIO_BASE_OFFSET;

#[derive(Debug)]
pub struct GPSET0;

impl Register<u32> for GPSET0 {
    const ADDRESS: *mut u32 = (MMIO_BASE_OFFSET + 0x0020001C) as *mut u32;
}

// GPIO 0-31
#[derive(Debug)]
#[repr(u32)]
pub enum GPSET0BitField {
    PIN00 = 0,
    PIN01 = 1,
    PIN02 = 2,
    PIN03 = 3,
    PIN04 = 4,
    PIN05 = 5,
    PIN06 = 6,
    PIN07 = 7,
    PIN08 = 8,
    PIN09 = 9,
    PIN10 = 10,
    PIN11 = 11,
    PIN12 = 12,
    PIN13 = 13,
    PIN14 = 14,
    PIN15 = 15,
    PIN16 = 16,
    PIN17 = 17,
    PIN18 = 18,
    PIN19 = 19,
    PIN20 = 20,
    PIN21 = 21,
    PIN22 = 22,
    PIN23 = 23,
    PIN24 = 24,
    PIN25 = 25,
    PIN26 = 26,
    PIN27 = 27,
    PIN28 = 28,
    PIN29 = 29,
    PIN30 = 30,
    PIN31 = 31,
}
