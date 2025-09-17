#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {} // TODO: Implement proper panic handling. 
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Our start.
    loop {}
}
