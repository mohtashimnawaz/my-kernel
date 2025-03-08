#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::panic::PanicInfo;
use my_kernel::{hlt_loop, init, vga_buffer};
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn start() -> ! {
    init();
    println!("Hello from Rust Kernel!");
    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic: {}", info);
    hlt_loop();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}