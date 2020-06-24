use crate::{bsp, cpu};
use cortex_a::{asm, regs::*};

/// # Safety
///
/// - Linker script must ensure to place this function at `0x80_000`.
#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    use crate::runtime_init;

    if bsp::cpu::BOOT_CORE_ID == cpu::smp::core_id() {
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