# lib-registers

---

# registers

See [`Cargo.toml`](Cargo.toml) `[features]` for all registers available.

### atmega328p

See [`src/registers/atmega328p/README.md`](src/registers/atmega328p/README.md) for further details.

All registers are available.

```toml
# enable feature in `Cargo.toml`
[dependencies]
lib-registers = { path = "/path/to/lib-registers", features = ["atmega328p"] }
```

```rust
// import `PORTB` related modules
use lib_registers::atmega328p::{
  DDRB, DDRBBitField,
  PORTB, PORTBBitField,
  PINB, PINBBitField,
};

```

### bcm

See [`src/registers/bcm/README.md`](src/registers/bcm/README.md) for further details.

Only the following registers are currently implemented:

- `GPFSEL*`
- `GPSET*`
- `GPCLR*`

```toml
# enable feature in `Cargo.toml`
[dependencies]
lib-registers = { path = "/path/to/lib-registers", features = ["bcm2835"] }
```

```rust
// import `GPFSEL0` module with tis bit fields
use lib_registers::bcm::{ GPFSEL0, GPFSEL0BitField };
```

---

#
