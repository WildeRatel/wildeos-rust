#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod wldserial;
mod wldvga;

use core::panic::PanicInfo;

// General purpose panic handler.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("{}", info);
    loop {}
}

// Panic test handler.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// Test runner.
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests.", tests.len() + 1);

    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}.....\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// Kernel main.
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

// Qemu Port setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
