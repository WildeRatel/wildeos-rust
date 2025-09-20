#![no_std]
#![no_main]

mod wldvga;

use core::panic::PanicInfo;

// General purpose panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
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
    println!("Testing put_char cursor return. This sentence aught to be on row 2.");

    loop {}
}
