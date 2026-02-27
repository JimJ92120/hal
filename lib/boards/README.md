# lib-boards

**Note**: `lib-boards` is using `lib-registers` crate as dependency (see [`lib/registers/README.md`](../registers/README.md) for further details).

---

# boards

See [`Cargo.toml`](Cargo.toml) `[features]` for all boards available.

### arduino uno

Following modules are implemented:

- `gpio`: minimal digital GPIO to set input / output, set output high / low
- `uart`: read and send via UART

```toml
# enable feature in `Cargo.toml`
[dependencies]
lib-boards = { path = "/path/to/lib-boards", features = ["arduino-uno"] }
```

```rust
// import `GPIO` module
use lib_boards::arduino_uno::{ GPIO }
```

### rpi

See [`src/boards/rpi/README.md`](src/boards/rpi/README.md) for further details on boards supported.

Following modules are implemented:

- `gpio`: minimal digital GPIO to set input / output, set output high / low

```toml
# enable feature in `Cargo.toml`
[dependencies]
lib-boards = { path = "/path/to/lib-boards", features = ["rpi3b1"] } # e.g Pi 3B v1.1
```

```rust
// import `GPIO` module
use lib_boards::rpi::{ GPIO }
```

---

#
