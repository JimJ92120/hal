// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=16
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct AUX_MU_LSR_REG;

impl Register<u32> for AUX_MU_LSR_REG {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x0021_5054) as *mut u32;
}

#[derive(Debug)]
#[repr(u32)]
pub enum AUX_MU_LSR_REG_BitField {
    DATA_READY          = 0,
    RECEIVER_OVERRUN    = 1,
    TRANSMITTER_EMPTY   = 5,
    TRANSMITTER_IDLE    = 6,
}
