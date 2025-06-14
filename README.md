# VersOS
A general purpose Unix-like operating system.

## Requirements
To build VersOS you'll need `cargo-make`, you can install it:
- with `cargo`, via `cargo install cargo-make`.
- with your distro package manager if you're on Linux (e.g. via `pacman -S cargo-make` on Arch).

## Build
You can build VersOS for two target, x86_64 and aarch64. To choose the target you have to define a profile to use with `cargo-make`.
For example for `x86_64` the command will be:
```
cargo make --profile x86_64 build
```

## Run in VM

The Makefile contains a script to run VersOS in a QEMU/KVM virtual machine.
```
cargo make --profile x86_64 run-qemu
```

By default, `cargo-make` will look for the UEFI firmware code and vars files in `/usr/share/ovmf/`. You can override these locations by setting `FIRMWARE_CODE` and `FIRMWARE_VARS` variables with the path of your firmware files.
