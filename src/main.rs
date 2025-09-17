#![no_std]
#![no_main]

mod wldvga;

use core::panic::PanicInfo;

use crate::wldvga::vga_greeting;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // TODO: Implement proper panic handling. 
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_greeting();

    loop {}
}
