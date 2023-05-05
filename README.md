# mos

## Build

```bash
# donwload std and core libs for embedded system(free standing environment)
rustup target add thumbv7em-none-eabihf 

# activate Rsut nightly release 
rustup toolchain add nightly-2022-04-01 --profile minimal
rustup override set nightly-2020-04-01
rustc --version # check `-nightly` exists on tail of version
rustup component add rust-src # install component to re-compile core, compiler-builins library
rustup component add llvm-tools-preview # install for building bootloader

# build os
cargo build

# build bootable image
carg bootimage

# build & run on QEMU
cargo run
```
