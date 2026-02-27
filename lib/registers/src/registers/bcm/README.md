# bcm

SoC registers are mapped at the same offset to the `MMIO_BASE` for the following models:

- BCM2835
- BCM2836
- BCM2837
- RP3A0
- BCM2837B0
- BCM2711

See [`Raspberry Pi processors`](https://www.raspberrypi.com/documentation/computers/processors.html) and [`Raspberry Pi models`](https://www.raspberrypi.com/news/raspberry-pi-product-series-explained/) for further details.

| MMIO_BASE    | SoC       | CPU          | arch      | support             | models                                  |
| ------------ | --------- | ------------ | --------- | ------------------- | --------------------------------------- |
| `0x20204000` | BCM2835   | ARM1176JZF-S | `ARMv6`   | `32-bit`            | Pi 1A, Pi 1A+, Pi 1B, Pi 1B+, Pi CM1    |
|              |           |              |           |                     | Pi Zero, Pi Zero W                      |
| `0x3F000000` | BCM2836   | Cortex-A7    | `ARMv7-A` | `32-bit`            | Pi 2B (`v1.1`)                          |
| `0x3F000000` | BCM2837   | Cortex-A53   | `ARMv8-A` | `32-bit` / `64-bit` | Pi 2B (`v1.2`), Pi 2B(`v1.3`)           |
|              |           |              |           |                     | Pi 3B (`v1.1`), Pi CM3                  |
| `0x3F000000` | RP3A0     | Cortex-A53   | `ARMv8-A` | `32-bit` / `64-bit` | Pi Zero 2, Pi Zero 2W                   |
| `0x3F000000` | BCM2837B0 | Cortex-A53   | `ARMv8-A` | `32-bit` / `64-bit` | Pi 3B (`v1.2`), Pi 3A+, Pi 3B+, Pi CM3+ |
| `0x7E000000` | BCM2711   | Cortex-A72   | `ARMv8-A` | `32-bit` / `64-bit` | Pi 4, Pi CM4, Pi CM4S, Pi 400,          |
| (tbd)        | BCM2712   | Cortex-A76   | `ARMv8-A` | `32-bit` / `64-bit` | Pi 5, Pi 500, Pi 500+, Pi CM5           |
| (tbd)        | BCM2712D0 | Cortex-A76   | `ARMv8-A` | `32-bit` / `64-bit` | Pi 5B                                   |

### differences

(to list possible different addresses, additional registers)

---

# references

- [`BCM2835`](https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf)
- [`BCM2837`](https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf)
- [`BCM2711`](https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf)
- https://www.raspberrypi.com/documentation/computers/processors.html
- https://www.raspberrypi.com/news/raspberry-pi-product-series-explained/
- https://developer.arm.com/documentation/ddi0301/latest/
