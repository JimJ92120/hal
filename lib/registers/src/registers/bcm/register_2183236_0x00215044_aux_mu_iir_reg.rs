// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=12
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct AUX_MU_IIR_REG;

impl Register<u32> for AUX_MU_IIR_REG {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x0021_5044) as *mut u32;
}

#[derive(Debug)]
#[repr(u32)]
pub enum AUX_MU_IIR_REG_BitField {
    ENABLE_TRANSMIT_INTERRUPT   = 0,
    ENABLE_RECEIVE_INTERRUPT    = 1,
}
