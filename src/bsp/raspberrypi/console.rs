// use crate::{console, synchronisation, synchronisation::NullLock};
// use core::fmt;

// struct QemuOutputInner {
//     chars_written: usize,
// }

// struct QemuOutput {
//     inner: NullLock<QemuOutputInner>,
// }

// static QEMU_OUTPUT: QemuOutput = QemuOutput::new();

// impl QemuOutputInner {
//     const fn new() -> QemuOutputInner {
//         QemuOutputInner { chars_written: 0 }
//     }

//     fn write_char(&mut self, c: char) {
//         unsafe {
//             core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
//         }

//         self.chars_written += 1;
//     }
// }

// impl fmt::Write for QemuOutputInner {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         for c in s.chars() {
//             if c == '\n' {
//                 self.write_char('\r');
//             }

//             self.write_char(c);
//         }

//         Ok(())
//     }
// }

// impl QemuOutput {
//     pub const fn new() -> QemuOutput {
//         QemuOutput {
//             inner: NullLock::new(QemuOutputInner::new())
//         }
//     }
// }

// pub fn console() -> &'static impl console::interface::All {
//     &QEMU_OUTPUT
// }

// use synchronisation::interface::Mutex;

// impl console::interface::Write for QemuOutput {
//     fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
//         let mut r = &self.inner;
//         r.lock(|inner| fmt::Write::write_fmt(inner, args))
//     }
// }

// impl console::interface::Statistics for QemuOutput {
//     fn chars_written(&self) -> usize {
//         let mut r = &self.inner;
//         r.lock(|inner| inner.chars_written)
//     }
// }

use super::memory;
use crate::{bsp::device_driver, console};
use core::fmt;

pub unsafe fn panic_console_out() -> impl fmt::Write {
    let mut uart = device_driver::PanicUart::new(memory::map::mmio::PL011_UART_BASE);
    uart.init();
    uart
}

pub fn console() -> &'static impl console::interface::All {
    &super::PL011_UART
}