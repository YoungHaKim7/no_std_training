int main(void)
{
    const char hello[] = "Hello, world!\n";

    // clang-format off
    __asm__ volatile (
       "mov x16, #4\n\t"         // syscall number for write on macOS ARM64
       "mov x0, #1\n\t"          // stdout (fd = 1)
       "mov x1, %0\n\t"          // address of message
       "mov x2, #14\n\t"         // length
       "svc #0\n\t"              // system call
        :
        : "r"(hello)
        : "x0", "x1", "x2", "x16"
    );
    // clang-format on

    return 0;
}
