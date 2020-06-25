pub mod console;
pub mod cpu;
pub mod driver;
pub mod memory;

// Global driver instances
use super::device_driver;

static GPIO: device_driver::GPIO =
    unsafe { device_driver::GPIO::new(memory::map::mmio::GPIO_BASE) };

static PL011_UART: device_driver::PL011Uart =
    unsafe { device_driver::PL011Uart::new(memory::map::mmio::PL011_UART_BASE) };

pub fn board_name() -> &'static str {
    "Raspberry Pi 3"
}