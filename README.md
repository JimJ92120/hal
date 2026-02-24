# hal

A `rust` bare-metal eco-system to map and interact with registers for various chips and boards.

---

# features

Following items can be enabled as `feature` in `Cargo.toml`.

### chip

- `atmega328p`: all registers are mapped
- `bcm2837`: `GPFSEL`, `GPSET`, `GPCLR` only

### boards

- `arduino-uno`: minimal digital GPIO to set input / output, set output high / low

---

# toolchains

See [`toolchains/README.md`](toolchains/README.md).

- `avr`

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

# usage

```sh
# run test
cargo run --manifest-path test/Cargo.toml

# build with toolchains
cd toolchains/$TARGET
nix-shell
cargo build --release
```

---

# reference
