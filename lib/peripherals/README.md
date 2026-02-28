# lib-peripherals

**Note**: `lib-peripherals` is using `lib-boards` crate as dependency (see [`lib/boards/README.md`](../boards/README.md) for further details).

---

# boards

See [`Cargo.toml`](Cargo.toml) `[features]` for all boards available.

### arduino uno

Following modules are implemented:

- `LED`: on, off, toggle, check if on

```toml
# enable feature in `Cargo.toml`
[dependencies]
lib-peripherals = { path = "/path/to/lib-peripherals", features = ["arduino-uno"] }
```

```rust
// import `led` module
use lib_peripherals::arduino_uno::{ Pin, LED }
```

---

#
