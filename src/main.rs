
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(naked_functions)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod memory;
mod panic_wait;
mod print;
mod runtime_init;

unsafe fn kernel_init() -> ! {
    println!("[0] Hello from Rust!");

    println!("[1] Stop here.");
    cpu::wait_forever()
}
