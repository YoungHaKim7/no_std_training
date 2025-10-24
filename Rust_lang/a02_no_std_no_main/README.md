# Result

```bash
$ rustup default nightly

$ rustc --target x86_64-unknown-linux-gnu -C panic=abort -C opt-level=z \
      -C link-args="-nostartfiles -nostdlib" \
      -o hello src/main.rs

$ ./hello
Hello, world!

```

# 다른 컴파일 방법

```bash
$ rustc -C linker=rust-lld -C link-self-contained=yes --target x86_64-unknown-linux-gnu \
      -C panic=abort -C opt-level=z -o hello src/main.rs
```

