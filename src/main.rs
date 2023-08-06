#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {
    println!("Xue Hua Piao Piao, Bi Feng Xiao Xiao{}", "!");
    panic!("Some panic message");
    loop {}
}

mod vga_buffer;
