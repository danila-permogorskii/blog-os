#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::fmt::Write;
use core::panic::PanicInfo;
mod vga_buffer;

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop{}
}
#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {

    }
}