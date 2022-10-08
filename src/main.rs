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

    aye_os::init();

    x86_64::instructions::interrupts::int3();

    // Cause page fault, as the virtual address 0xdeadbeef isn't mapped to a physical address
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // }

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

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
