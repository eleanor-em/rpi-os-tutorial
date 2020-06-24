use core::ops::Range;

/// # Safety:
///
/// - `range.start`, `range.end` valid
/// - `range.start`, `range.end` are `T`-aligned
pub unsafe fn zero_volatile<T>(range: Range<*mut T>)
        where T: From<u8> {
    let mut ptr: *mut T = range.start;

    while ptr < range.end {
        core::ptr::write_volatile(ptr, T::from(0));
        ptr = ptr.offset(1);
    }
}