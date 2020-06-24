use crate::memory;
use core::ops::Range;

/// # Safety
///
/// - The symbol-provided addresses must be valid.
/// - The symbol-provided addresses must be usize aligned.
unsafe fn bss_range() -> Range<*mut usize> {
    extern "C" {
        // Boundaries of the .bss section, provided by linker script symbols.
        static mut __bss_start: usize;
        static mut __bss_end: usize;
    }

    Range {
        start: &mut __bss_start,
        end: &mut __bss_end,
    }
}

/// # Safety
///
/// - Must only be called pre `kernel_init()`.
#[inline(always)]
unsafe fn zero_bss() {
    memory::zero_volatile(bss_range());
}

/// # Safety
///
/// - Only a single core must be active and running this function.
pub unsafe fn runtime_init() -> ! {
    zero_bss();

    crate::kernel_init()
}
