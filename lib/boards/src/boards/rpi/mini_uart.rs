use lib_registers::{
    globals::{ Register },
    bcm::{
        AUX_ENABLES, AUX_ENABLES_BitField,
        AUX_MU_CNTL_REG, AUX_MU_CNTL_REG_BitField,
        AUX_MU_LCR_REG, AUX_MU_LCR_REG_BitField,
        AUX_MU_MCR_REG,
        AUX_MU_IER_REG, AUX_MU_IIR_REG_BitField,
        AUX_MU_IIR_REG,
        AUX_MU_BAUD_REG,
        AUX_MU_IO_REG,
        AUX_MU_LSR_REG, AUX_MU_LSR_REG_BitField,
    }
};

#[derive(Debug)]
pub struct MiniUART;

impl MiniUART {
    pub fn init(baud_rate: u32, frequency: u32) {
        AUX_ENABLES::or(1 << AUX_ENABLES_BitField::MINI_UART_ENABLE as u32);

        AUX_MU_IIR_REG::or(AUX_MU_IIR_REG_BitField::ENABLE_TRANSMIT_INTERRUPT as u32);
        AUX_MU_LCR_REG::or(1 << AUX_MU_LCR_REG_BitField::DATA_SIZE as u32);
        // dynamic frequency changes not accounted
        AUX_MU_BAUD_REG::set(Self::calculate_baud_rate_from_frequency(frequency, baud_rate));

        AUX_MU_CNTL_REG::set(0);
        AUX_MU_MCR_REG::set(0);
        AUX_MU_IER_REG::set(0);

        AUX_MU_CNTL_REG::or(1 << AUX_MU_CNTL_REG_BitField::TRANSMITTER_ENABLE as u32);
    }

    pub fn send(content: &str) {
        for byte in content.as_bytes() {
            Self::send_byte(*byte);

            if b'\n' == *byte {
                Self::send_byte(b'\r');
            }
        }
    }

    fn send_byte(byte: u8) {
        while 0 == AUX_MU_LSR_REG::get() & (1 << AUX_MU_LSR_REG_BitField::TRANSMITTER_EMPTY as u32) {}

        AUX_MU_IO_REG::set(byte.into());
    }

    fn read() -> u8 {
        while 0 == AUX_MU_LSR_REG::get() & (1 << AUX_MU_LSR_REG_BitField::DATA_READY as u32) {}

        AUX_MU_IO_REG::get() as u8
    }

    fn push(content: &str) {
        for byte in content.as_bytes() {
            Self::send_byte(*byte);

            if b'\n' == *byte {
                Self::send_byte(b'\r');
            }
        }
    }

    fn calculate_baud_rate_from_frequency(frequency: u32, baud_rate: u32) -> u32 {
        (frequency / (frequency / 1_000_000) / baud_rate) - 1
    }
}
