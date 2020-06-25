// = ../cpu.rs in tutorial

#[cfg(target_arch = "aarch64")]
pub mod smp;

use crate::bsp;
use cortex_a::{asm, regs::*};

pub use asm::nop;

#[inline(always)]
pub fn spin_for_cycles(n: usize) {
    for _ in 0..n {
        asm::nop()
    }
}

/// # Safety
///
/// - Linker script must ensure to place this function at `0x80_000`.
#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    use crate::runtime_init;

    if bsp::cpu::BOOT_CORE_ID == crate::cpu::smp::core_id() {
        SP.set(bsp::cpu::BOOT_CORE_STACK_START);
        runtime_init::runtime_init()
    } else {
        wait_forever()
    }
}

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}