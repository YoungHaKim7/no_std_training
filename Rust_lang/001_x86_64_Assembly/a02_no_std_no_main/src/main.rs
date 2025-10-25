#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use core::arch::asm;

/// Entry point (no std, no runtime)
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Fixed-size message
    let msg = b"Hello, world!\n";
    let len = msg.len(); // runtime variable OK here

    unsafe {
        // write(1, msg, len)
        asm!(
            "mov rax, 1",          // syscall: write
            "mov rdi, 1",          // fd = stdout
            "mov rsi, {msg}",      // pointer to message
            "mov rdx, {len}",      // length
            "syscall",
            msg = in(reg) msg.as_ptr(),
            len = in(reg) len,
            out("rax") _, out("rdi") _, out("rsi") _, out("rdx") _,
        );

        // exit(0)
        asm!(
            "mov rax, 60",         // syscall: exit
            "xor rdi, rdi",        // status = 0
            "syscall",
            options(noreturn),
        );
    }
}

/// Required panic handler for no_std
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
