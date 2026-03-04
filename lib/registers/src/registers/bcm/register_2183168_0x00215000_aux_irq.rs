// https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf#page=9
use crate::globals::Register;
use super::MMIO_BASE;

#[derive(Debug)]
pub struct AUX_IRQ;

impl Register<u32> for AUX_IRQ {
    const ADDRESS: *mut u32 = (MMIO_BASE + 0x0021_5000) as *mut u32;
}

#[derive(Debug)]
#[repr(u32)]
pub enum AUX_IRQ_BitField {
    SPI_2_IRQ       = 0,
    SPI_1_IRQ       = 1,
    MINI_UART_IRQ   = 2,
}
