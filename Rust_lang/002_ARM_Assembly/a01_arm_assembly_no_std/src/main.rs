#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Required panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// macOS’s dynamic linker looks for this symbol unless you fully disable dynamic linking
#[unsafe(no_mangle)]
pub extern "C" fn dyld_stub_binder() {}

// Required by LLVM-generated code for exception unwinding, even if unused
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

// Some compiler intrinsics may call this (dummy impl)
#[unsafe(no_mangle)]
pub extern "C" fn _Unwind_Resume() {}

// Entry point (_start replaces main)
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let x = 42;
    let y = 24;
    let _result = x + y; // 66

    loop {}
}
