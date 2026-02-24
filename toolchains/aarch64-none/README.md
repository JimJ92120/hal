# hal-toolchains-aarch64-none

Toolchain boilerplate for `aarch64-none` (bare-metal) with a basic LED blink example to **GPIO 27**.

Examples will target **Raspberry Pi 3B** (`BCM2837`).  
See below to target other models.

---

# installation

### requirements

|                             |                                                     |
| --------------------------- | --------------------------------------------------- |
| `rust`                      | `nighlty` (`1.95.0-nightly (838709580 2026-02-17)`) |
| `cargo`                     | `nighlty` (`1.95.0-nightly (838709580 2026-02-17)`) |
| `aarch64-none-elf-gcc`      | `14.3.0`                                            |
| `binutils-aarch64-none-elf` |                                                     |
| `qemu-system-aarch64`       | `10.1.2`                                            |

### nix

Build dependencies and environment are setup in `shell.nix`.

```sh
nix-shell
```

Note that after running `nix-collect-garbage -d`, project may still refer to old (non-existing) `/nix-store` paths for `binutils`.  
Delete `~/.rustup` and `~/.cargo` directory if so, then exit and re-enter `nix-shell`.

---

# usage

```sh
# build rust
cargo build --release

# inspect build
file target/aarch64-unknown-none/release/hal-toolchains-aarch64-none
aarch64-none-elf-objdump -d -D target/aarch64-unknown-none/release/hal-toolchains-aarch64-none >> build/aarch64.s

# build kernel8.img
aarch64-none-elf-objcopy -O binary target/aarch64-unknown-none/release/hal-toolchains-aarch64-none build/kernel8.img

# qemu
qemu-system-aarch64 -M help | grep raspi
qemu-system-aarch64 -M raspi3b -kernel build/kernel8.img -serial null -serial stdio -display none
```

### qemu

`qemu-system-aarch64` have a limited set of available machine for Raspberry Pi's models.  
Run `qemu-system-aarch64 -M help | grep raspi` to see supported models.

---

# sd card

1. create a bootable `FAT32` partion
2. copy [`raspberry/firmware`](https://github.com/raspberrypi/firmware/) files into the created partition (may change based on model):
   - [`bootcode.bin`](https://github.com/raspberrypi/firmware/blob/master/boot/bootcode.bin)
   - [`fixup.dat`](https://github.com/raspberrypi/firmware/blob/master/boot/fixup.dat)
   - [`start.elf`](https://github.com/raspberrypi/firmware/blob/master/boot/start.elf)
3. copy `build/kernel8.img` into the created partition

---

# different models

Add the corresponding `feature` in `Cargo.toml`:

```toml
[dependencies]
hal-lib = { path = "../../lib", features = [$TARGET] }
```

SD card preparation may require different `/boot` files (e.g `start4.elf` instead of `start.elf` for Pi 4).

`src/boot/boot.s` may also require changes.

---

# references
