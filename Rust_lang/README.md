# Rust(no_std)
- The Embedded Rust Book: An introductory book about using the Rust Programming Language on "Bare Metal" embedded systems, such as Microcontrollers.
  - https://docs.rust-embedded.org/book/intro/index.html

# Getting Started with Rust on a Raspberry Pi Pico (Part 1)
- https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry
  - https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf

```
$ git clone https://github.com/raspberrypi/debugprobe.git
$ cd debugprobe
$ git submodule update --init
$ mkdir build
$ cd build
$ export PICO_SDK_PATH=../../pico-sdk
$ cmake -DDEBUG_ON_PICO=ON -DPICO_BOARD=pico ..
$ make -j4
```
