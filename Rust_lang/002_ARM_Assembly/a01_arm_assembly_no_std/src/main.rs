#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// For macOS compatibility
#[cfg(target_os = "macos")]
#[unsafe(no_mangle)]
pub extern "C" fn _dyld_private_extern() {}

#[cfg(target_os = "macos")]
#[unsafe(no_mangle)]
pub extern "C" fn dyld_stub_binder() {}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() -> ! {
    loop {}
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // This is a minimal no_std main function
    // We can't use println! because it requires std

    // Create some simple calculations to show it works
    let x = 42;
    let y = 24;
    let result = x + y;

    // Since we can't print, we'll just demonstrate the logic
    let _ = result; // This would be 66

    // In a real embedded system, you might:
    // - Write to memory-mapped I/O registers
    // - Call platform-specific functions
    // - Use assembly to trigger hardware actions

    loop {
        // Infinite loop - typical for embedded systems
        // In real applications, you might have:
        // - Sleep/wake cycles
        // - Interrupt handling
        // - State machines
    }
}
