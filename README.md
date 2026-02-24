# hal

A `rust` bare-metal eco-system to map and interact with registers for various chips and boards.

---

# features

- `atmega328p`

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
