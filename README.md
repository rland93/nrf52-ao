# nrf52-ao 

I wanted to understand three things:

- how an active object framework is implemented
- how a mixed C/Rust codebase works
- how to use the Nordic SoftDevice on nRF52

So, I created this firmware. Rather than the "out of the box" experience of
embassy-rs or even rtic with cortex-m and a HAL and PAC, etc, this firmware uses
the nrfx driver package and startup code from the nrf SDK; then, we have a Rust
library to act as the application layer on top of that.

The firmware is written without basically any hardware abstraction, it's totally
built for nRF52840 processor. No effort is made for hardware abstraction.

## this repository

```
.
├── build
├── c         <---- C source code
├── Cargo.lock
├── Cargo.toml
├── cmake
├── CMakeLists.txt
├── compile_commands.json
├── Makefile
├── README.md
├── rust-toolchain.toml
├── src        <---- rust application source code
├── target     <---- (created by Cargo) location of the rust binary
├── toolchain  <---- gcc toolchain
└── vendor     <---- all of nordic's code, CMSIS, etc. as git submodules
```

## build

### prerequisites

- cmake
- rustup

(1) pull down nrfx driver code:

```bash
git submodule update --init --recursive
```

(2) download the toolchain:

```bash
chmod +x ./gcc-toolchain.sh
./gcc-toolchain.sh
```

note. You'll have to run this script twice. Once to acquire the SHA and then,
once you have the SHA, you paste it into the script and run again to actually
download. This does not modify your system files and should work for MacOS and
Linux.

(3) run cmake

```bash
mkdir build
cd build
cmake ..
```

(4) compile

```bash
cmake --build build
```

To run it, I have a debug setup for probe-rs in the .zed directory.

## nRF52

This firmware uses the NVIC of the nRF52 for concurrency.

We have 8 different priorities available to us. Of these, 3 are owned by the
bluetooth softdevice. They are split up as follows:


```
P0 -> owned by softdevice
P1 -> owned by softdevice
P2 -> high priority interrupts
P3 -> high priority active objects
P4 -> owned by softdevice (softdevice events)
P5 -> low priority interrupts
P6 -> low priority active objects
P7 -> low priority active objects
-  -> main() (sleep)
```

# Use of AI

I am using the AI primarily for code review and bouncing ideas. My rule is that
I have to type everything in and also I have to understand everything that I'm
typing in. None of the code here is written by 'vibes'
