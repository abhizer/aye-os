#![feature(custom_test_frameworks)]
#![test_runner(aye_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

use aye_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    aye_os::test_panic_handler(info)
}
