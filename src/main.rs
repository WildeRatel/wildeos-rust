#![no_std]
#![no_main]

mod wldvga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // TODO: Implement proper panic handling. 
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    wldvga::WRITER.lock().vga_paint();
    wldvga::WRITER.lock().write_string("Welcome to wildeos!");

    loop {}
}
