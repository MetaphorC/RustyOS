#![no_std]
#![no_main]

extern crate rusty_os;

use core::panic::PanicInfo;
use rusty_os::{QemuExitCode, exit_qemu, serial_println, serial_print};


#[no_mangle]
pub extern "C" fn _start() -> ! {
   should_fail();
    serial_println!("[test did no panic]");
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop{}
}

