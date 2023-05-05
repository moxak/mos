# mos

## Build

```bash
# donwload std and core libs for embedded system(free standing environment)
rustup target add thumbv7em-none-eabihf 

# activate Rsut nightly release 
rustup toolchain add nightly-2020-07-20 --profile minimal
rustup override set nightly-2020-07-20
rustc --version # check `-nightly` exists on tail of version
rustup component add rust-src # install rust-src component re-compile core, compiler-builins library

# build os
cargo build --target thumbv7em-none-eabihf
```
