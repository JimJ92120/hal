// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=13
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct AUX_MU_IER_REG;

impl Register<u32> for AUX_MU_IER_REG {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x0021_5048) as *mut u32;
}

#[derive(Debug)]
#[repr(u32)]
pub enum AUX_MU_IER_REG_BitField {
    INTERRUPT_PENDING           = 0,
    READ_INTERRUPT_WRITE_FIFO   = 1, // 1:2
    FIFO_ENABLES                = 6, // 6:7
}
