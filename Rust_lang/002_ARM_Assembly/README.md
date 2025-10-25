# `rustc`


```bash
⎿      Running `/Users/gy-gyoung/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rustc --crate-name a01_arm_assembly_no_std --edition=2024
    src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C panic=abort
    -C lto -C debuginfo=2 -C split-debuginfo=unpacked --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C
    metadata=19149397809e038f -C extra-filename=-0c295af693784f7c --out-dir
    /Users/gy-gyoung/my_project/Rust_Lang/9999/2222/no_std_training/Rust_lang/002_ARM_Assembly/a01_arm_assembly_no_std/target/debug/deps -C incremen
    tal=/Users/gy-gyoung/my_project/Rust_Lang/9999/2222/no_std_training/Rust_lang/002_ARM_Assembly/a01_arm_assembly_no_std/target/debug/incremental
    -L
    dependency=/Users/gy-gyoung/my_project/Rust_Lang/9999/2222/no_std_training/Rust_lang/002_ARM_Assembly/a01_arm_assembly_no_std/target/debug/deps
    --extern panic_halt=/Users/gy-gyoung/my_project/Rust_Lang/9999/2222/no_std_training/Rust_lang/002_ARM_Assembly/a01_arm_assembly_no_std/target/de
    bug/deps/libpanic_halt-ab98aa64e1ad87a8.rlib -C link-arg=-Wl,-e,_start -C link-arg=-Wl,-undefined,dynamic_lookup`
      process didn't exit successfully: `/Users/gy-gyoung/.rustup/toolchains/stable-aarch64-apple-darwin/bin/rustc --crate-name
    a01_arm_assembly_no_std --edition=2024 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type
    bin --emit=dep-info,link -C panic=abort -C lto -C debuginfo=2 -C split-debuginfo=unpacked --check-cfg 'cfg(docsrs,test)' --check-cfg
    'cfg(feature, values())' -C metadata=19149397809e038f -C extra-filename=-0c295af693784f7c --out-dir
    /Users/gy-gyoung/my_project/Rust_Lang/9999/2222/no_std_training/Rust_lang/002_ARM_Assembly/a01_arm_assembly_no_std/target/debug/deps -C incremen
    tal=/Users/gy-gyoung/my_project/Rust_Lang/9999/2222/no_std_training/Rust_lang/002_ARM_Assembly/a01_arm_assembly_no_std/target/debug/incremental
    -L
    dependency=/Users/gy-gyoung/my_project/Rust_Lang/9999/2222/no_std_training/Rust_lang/002_ARM_Assembly/a01_arm_assembly_no_std/target/debug/deps
    --extern panic_halt=/Users/gy-gyoung/my_project/Rust_Lang/9999/2222/no_std_training/Rust_lang/002_ARM_Assembly/a01_arm_assembly_no_std/target/de
    bug/deps/libpanic_halt-ab98aa64e1ad87a8.rlib -C link-arg=-Wl,-e,_start -C link-arg=-Wl,-undefined,dynamic_lookup` (exit status: 1)

$ rustc --crate-type bin src/main.rs --emit=obj -o main.o

```

# nm

```
   nm main.o | grep _start
   Check if _start symbol exists in object file

⏺ Bash(nm main.o | grep dyld)
  ⎿  No matches found

⏺ Bash(rm main.o && rustc --crate-type bin src/main.rs --emit=obj -o main.o -C panic=abort)
```

# QEMU(Arm edition)

- [(230623) macOS in QEMU (ARM edition)](https://youtu.be/oZqFYJVOUQo?si=An5JXjNu06XbQo8p)

|XNU Kernel<br />t6000|XNU Kernel<br />t6020|XNU Kernel<br />t8103|XNU Kernel<br />t8112|XNU Kernel<br />VMApple|
|-|-|-|-|-|
|M1 Pro<br />M1 Max<br />M1 Ultra|M1 Pro<br />M1 Max<br />M1 Ultra|M1|M2|Virtualization.<br />Framework|
