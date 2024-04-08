// Entry Point for PilotOS

#![no_std] // Disable linking the Rust standard library.
#![no_main] // Remove main as the entry point.

use core::panic::PanicInfo;
 

// This function is called, whenever a panic is triggered.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // Disable mangling to preserve function name. 
// The _start function will be used as an entry point by the crt0 runtime.
// Use C calling Convention (This funciton is called by C runtime)
// This function is Diverging and never returns.
pub extern "C" fn _start() -> ! {
    loop {
    }
}