#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod wldvga;

use core::panic::PanicInfo;

// General purpose panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests.", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("Trivial assertion: ");
    assert_eq!(1, 1);
    println!("[ok]")
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    wldvga::WRITER.lock().vga_paint();
    println!(
        "Welcome to wildeos!\nThe awnser to life, the universe and everything is: {}",
        42
    );
    wldvga::WRITER
        .lock()
        .put_char(b'+', wldvga::BUFFER_WIDTH / 2, wldvga::BUFFER_HEIGHT / 2);
    println!(
        "Testing put_char cursor return. This sentence aught to be on row 2 spanning all the way to 3 (Rows start at 0)."
    );
    println!();

    #[cfg(test)]
    test_main();

    loop {}
}
