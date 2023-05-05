# mos

## Build

```bash
# donwload std and core libs for embedded system(free standing environment)
rustup target add thumbv7em-none-eabihf 

# activate Rsut nightly release 
rustup install nightly
rustup override set nightly
rustc --version # check `-nightly` exists on tail of version

# build os
cargo build --target thumbv7em-none-eabihf
```
