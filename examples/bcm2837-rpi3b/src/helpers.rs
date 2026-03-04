use core::arch::asm;

pub fn delay(cycles: u32) {
    unsafe {
        for _ in 0..cycles {
            asm!("nop");
        }
    }
}
