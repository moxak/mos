# mos

## Build

```bash
# donwload std and core libs for embedded system(free standing environment)
rustup target add thumbv7em-none-eabihf 

# build os
cargo build --target thumbv7em-none-eabihf
```
