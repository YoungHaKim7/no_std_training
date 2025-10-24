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



