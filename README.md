# hal

Some utilities to map and interact with registers for various chips and boards in `rust`.

---

# features

Following items can be enabled as `[features]` defined in `Cargo.toml`.  
See [`lib/Cargo.toml`](lib/Cargo.toml) for all defined features and dependencies tree.

### registers

- `atmega328p`: all registers are mapped
- `bcm*`: `GPFSEL`, `GPSET`, `GPCLR` only (see [`lib/src/registers/bcm/README.md`](lib/src/registers/bcm/README.md))

### boards

- `arduino-uno`: minimal digital GPIO to set input / output, set output high / low
- `rpi-*`: minimal digital GPIO to set input / output, set output high / low (see [`lib/src/boards/rpi/README.md`](lib/src/boards/rpi/README.md))

---

# toolchains

Following toolchains are available with a minimal example to consume `hal-lib`.

- [`avr`](toolchains/avr/README.md)
- [`aarch64-none`](toolchains/aarch64-none/README.md)
- [`arm-none`](toolchains/arm-none/README.md)

```sh
# build with toolchains
$TOOLCHAIN=avr

cd toolchains/$TOOLCHAIN

nix-shell

cargo build --release
```

---

# installation

### requirements

|         |                                                     |
| ------- | --------------------------------------------------- |
| `rust`  | `nighlty` (`1.95.0-nightly (838709580 2026-02-17)`) |
| `cargo` | `nighlty` (`1.95.0-nightly (838709580 2026-02-17)`) |

### nix

Build dependencies and environment are setup in `shell.nix`.

```sh
nix-shell
```

---

# reference
