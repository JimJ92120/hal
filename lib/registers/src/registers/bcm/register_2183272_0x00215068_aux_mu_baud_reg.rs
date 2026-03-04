// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=19
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct AUX_MU_BAUD_REG;

impl Register<u32> for AUX_MU_BAUD_REG {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x0021_5068) as *mut u32;
}

#[derive(Debug)]
#[repr(u32)]
pub enum AUX_MU_BAUD_REG_BitField {
    BAUD_RATE = 0, // 0:15
}
