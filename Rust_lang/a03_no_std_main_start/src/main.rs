#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use core::arch::asm;

/// Entry point — like `main()` but without std runtime.
#[unsafe(no_mangle)]
pub fn main() -> ! {
    const MSG:&[u8; 14] = b"Hello, world!\n";

    unsafe {
        // syscall: write(stdout, msg, len)
        asm!(
            "mov rax, 1",          // SYS_write
            "mov rdi, 1",          // fd = 1 (stdout)
            "mov rsi, {MSG}",      // pointer to buffer
            "mov rdx, {len}",      // length
            "syscall",
            MSG = in(reg) MSG.as_ptr(),
            len = const MSG.len(),
            out("rax") _, out("rdi") _, out("rsi") _, out("rdx") _,
        );

        // syscall: exit(0)
        asm!(
            "mov rax, 60",         // SYS_exit
            "xor rdi, rdi",        // exit code = 0
            "syscall",
            options(noreturn),
        );
    }
}

/// Required language items (so linking works without std)
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// #[lang = "eh_personality"] extern "C" fn eh_personality() {}
