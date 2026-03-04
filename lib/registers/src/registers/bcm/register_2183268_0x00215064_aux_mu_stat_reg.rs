// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=18
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct AUX_MU_STAT_REG;

impl Register<u32> for AUX_MU_STAT_REG {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x0021_5064) as *mut u32;
}

#[derive(Debug)]
#[repr(u32)]
pub enum AUX_MU_STAT_REG_BitField {
    SYMBOL_AVAILABLE            = 0,
    SPACE_AVAILABLE             = 1,
    RECEIVER_IS_IDLE            = 2,
    TRANSMITTER_IS_IDLE         = 3,
    RECEIVER_OVERRUN            = 4,
    TRANSMIT_FIFO_FULL          = 5,
    RTS_STATUS                  = 6,
    CTS_LINE                    = 7,
    TRANSMIT_FIFO_EMPTY         = 8,
    TRANSMITTER_DONE            = 9,
    RECEIVE_FIFO_FILL_LEVEL     = 16, // 16:19
    TRANSMIT_FIFO_FILL_LEVEL    = 24, // 24:27
}
