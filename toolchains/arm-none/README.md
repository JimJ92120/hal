# toolchains-arm-none

Toolchain boilerplate for `arm-none` (bare-metal) with a basic LED blink example to **GPIO 27**.

Examples will target **Raspberry Pi 2B v1.1** (`BCM2836`).  
See below to target other models.

---

# installation

### requirements

|                          |                                                     |
| ------------------------ | --------------------------------------------------- |
| `rust`                   | `nighlty` (`1.95.0-nightly (838709580 2026-02-17)`) |
| `cargo`                  | `nighlty` (`1.95.0-nightly (838709580 2026-02-17)`) |
| `arm-none-eabi-gcc`      | `14.3.0`                                            |
| `binutils-arm-none-eabi` |                                                     |
| `qemu-system-arm`        | `10.1.2`                                            |

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
file target/armv7a-none-eabi/release/hal-toolchains-arm-none
arm-none-eabi-objdump -d -D target/armv7a-none-eabi/release/hal-toolchains-arm-none >> build/armv7a.s

# build kernel7.img
arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/release/hal-toolchains-arm-none build/kernel7.img

# qemu
qemu-system-arm -M raspi2b -kernel build/kernel7.img -serial null -serial stdio -display none
```

### qemu

`qemu-system-arm` have a limited set of available machine for Raspberry Pi's models.  
Run `qemu-system-arm -M help | grep raspi` to see supported models.

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
