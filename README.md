# hal

Some utilities to map and interact with registers for various chips and boards in `rust`.

---

# lib

Different utilities are available, based on use case and abstraction level needed.
Each utility has its own crate (see `lib/` directory) with various `[features]` defined.

The following crates are available:

### registers

Map processors, chips registers addresses and their bit fields.

See [`lib/registers/README.md`](lib/registers/README.md).

```toml
# enable in Cargo.toml
[dependencies]
lib-registers = { path = "/path/to/lib/registers", features = [] }
```

### boards

Abstract `lib/registers` for specific boards (e.g Arduino, Raspberry Pi).

See [`lib/boards/README.md`](lib/boards/README.md).

```toml
# enable in Cargo.toml
[dependencies]
lib-registers = { path = "/path/to/lib/boards", features = [] }
```

---

# examples

Some examples using `lib/boards` and `lib/peripherals` are available in `examples/` directory.
See [`examples/README.md`](examples/README.md) for further details.

See [`nix-rust-embedded`](https://github.com/JimJ92120/nix-rust-embedded) for toolchains and environments.

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
