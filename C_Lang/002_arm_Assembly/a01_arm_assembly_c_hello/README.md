# Result

```bash
$ /opt/homebrew/opt/gcc@15/bin/gcc-15 -std=c23 -pedantic -pthread -pedantic-errors -lm -Wall -Wextra -ggdb -Werror -o ./target/a01_arm_assembly_c_hello ./src/main.c

$ ./target/a01_arm_assembly_c_hello

Hello, world!
```


# ARM64
The output shows "Hello, world!" which means the inline assembly system call is working properly on macOS ARM64.

- ⏺ Summary
  - The SIGSYS error was caused by using incorrect register and syscall conventions for macOS ARM64. The key fixes were:
    - 1. Register selection: Changed from w8 to x16 for the syscall number register
    - 2. Register sizing: Used 64-bit registers (x0, x1, x2, x16) instead of 32-bit (w0, w1, etc.)
    - 3. Syscall number: Kept #4 which is correct for the write system call on macOS
    - 4. Clobber list: Updated to match the registers actually used
- The corrected inline assembly now properly makes a system call to write() on macOS ARM64, successfully printing "Hello, world!" without causing a SIGSYS error.
