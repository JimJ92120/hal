# examples

**Note**: `examples` is using [`lib-boards`](../lib/boards/README.md) and [`lib-peripherals`](../lib/peripherals/README.md) crates as dependencies.

---

# examples

`examples/` directory contains different examples using different boards and processors.
Each example can be copied / pasted into a project (with required dependencies) or imported as described in below sections.

### atmega328p-arduino-uno

See [`bcm2837-rpi3b/Cargo.toml`](bcm2837-rpi3b/Cargo.toml) for required dependencies.

Following examples are implemented:

- led_blink
- button_toggle
- uart_send
- uart_read
- rgb_led
- analog_input
- i2c_lcd

```toml
# in Cargo.toml
[dependencies]
atmega328p-arduino-uno = { path = "path/to/examples/atmega328p-arduino-uno" }
```

```rust
// in src/main.rs
use atmega328p_arduino_uno::examples::{ boards, peripherals };

fn main() {
    peripherals::led_blink::run();
}
```

### bcm2837-rpi3b

See [`bcm2837-rpi3b/Cargo.toml`](bcm2837-rpi3b/Cargo.toml) for required dependencies.

Following examples are implemented:

- led_blink
- mini_uart

```toml
# in Cargo.toml
[dependencies]
bcm2837-rpi3b = { path = "path/to/examples/bcm2837-rpi3b" }
```

```rust
// in src/main.rs
use bcm2837_rpi3b::examples::{ led_blink };

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    led_blink::run();
}

```

---

# references
