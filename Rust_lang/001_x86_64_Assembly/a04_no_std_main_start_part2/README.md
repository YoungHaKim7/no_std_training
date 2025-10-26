# Key Features:
- 1. #![no_std] - Disables the standard library
- 2. #![no_main] - Tells Rust not to use the usual main function entry point
  - 3. Panic Handler - Custom panic handler since we can't use the default one
  - 4. #[unsafe(no_mangle)] - Ensures the function name isn't mangled by the compiler
  5. extern "C" - Uses C calling convention for platform compatibility
  6. -> ! - Never return type (diverges)

  How it works:

  The example demonstrates a minimal embedded-style program that:
  - Performs simple arithmetic (42 + 24 = 66)
  - Runs in an infinite loop (typical for embedded systems)
  - Uses only the core library (not std)
  - Includes proper error handling with a panic abort strategy

  This is a trustworthy no_std foundation that can be extended for embedded systems, kernel development, or other bare-metal
  applications.

# Result

```bash
❯ rustc +nightly --target x86_64-unknown-linux-gnu \
      -C panic=abort -C opt-level=z \
      -C link-args="-nostartfiles -nostdlib" \
       -o hello src/main.rs

```


# 용량 더 작게

🧩 Optional: Make it even smaller

To make a fully static, minimal ELF (~5 KB total):

```bash
rustc +nightly --target x86_64-unknown-linux-gnu \
    -C panic=abort -C opt-level=z -C link-self-contained=yes -C linker=rust-lld \
    -C relocation-model=static \
    -C link-args="-nostartfiles -static" \
    -o hello src/main.rs

```
