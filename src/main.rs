#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate rusty_os;

use core::panic::PanicInfo;
use rusty_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hellow World{}", "!");

    #[cfg(test)]
    test_main();
    loop{}
}

/// This fn is called on panic 
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusty_os::test_panic_handler(info)
}
