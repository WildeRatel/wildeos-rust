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
    wldvga::WRITER.lock().write_string("Welcome to wildeos!\n");
    wldvga::WRITER
        .lock()
        .put_char(b'+', wldvga::BUFFER_WIDTH / 2, wldvga::BUFFER_HEIGHT / 2);
    wldvga::WRITER.lock().write_string("TESTING!");

    loop {}
}
