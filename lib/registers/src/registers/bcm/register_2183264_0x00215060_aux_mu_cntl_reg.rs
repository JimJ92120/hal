// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=16
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct AUX_MU_CNTL_REG;

impl Register<u32> for AUX_MU_CNTL_REG {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x0021_5060) as *mut u32;
}

#[derive(Debug)]
#[repr(u32)]
pub enum AUX_MU_CNTL_REG_BitField {
    RECEIVER_ENABLE             = 0,
    TRANSMITTER_ENABLE          = 1,
    ENABLE_RECEIVE_AUTO_RTS     = 2,
    ENABLE_TRANSMIT_AUTO_CTS    = 3,
    RTS_AUTO_FLOW_LEVEL         = 4, // 4:5
    RTS_ASSERT_LEVEL            = 6,
    CTS_ASSERT_LEVEL            = 7,
}
