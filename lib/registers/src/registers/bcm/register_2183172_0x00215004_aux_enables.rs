// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=9
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct AUX_ENABLES;

impl Register<u32> for AUX_ENABLES {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x0021_5004) as *mut u32;
}

#[derive(Debug)]
#[repr(u32)]
pub enum AUX_ENABLES_BitField {
    SPI_2_ENABLE        = 0,
    SPI_1_ENABLE        = 1,
    MINI_UART_ENABLE    = 2,
}
