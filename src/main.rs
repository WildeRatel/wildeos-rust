#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(wildeos_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod wldvga;

// General purpose panic handler.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Test handler.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    wildeos_rust::test_panic_handler(info);
}

// Kernel main.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    wldvga::WRITER.lock().vga_paint();
    println!(
        "Welcome to wildeos!\nThe awnser to life, the universe and everything is: {}",
        42
    );

    #[cfg(test)]
    test_main();

    loop {}
}
