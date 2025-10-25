use std::arch::asm;

fn main() {
    let hello = "Hello, world!\n";

    unsafe {
        asm!(
            "mov rax, 1",          // syscall: write
            "mov rdi, 1",          // fd = stdout
            "lea rsi, [{msg}]",    // pointer to message
            "mov rdx, 14",         // length
            "syscall",             // invoke kernel
            msg = in(reg) hello.as_ptr(),
            out("rax") _, out("rdi") _, out("rsi") _, out("rdx") _,
        );
    }
}
