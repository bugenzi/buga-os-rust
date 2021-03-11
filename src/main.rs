#![no_std]// dont lin the rust lib 
#![no_main] // disable all Rust-level entry points 

use core::panic::PanicInfo;
mod vga_buffer;
/// This function is called on panic.

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  

 // First version 

    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    vga_buffer::print_something();
    loop {}
}

#[panic_handler] 
fn panic    (_info: &PanicInfo) -> ! {
    
    loop {}
}