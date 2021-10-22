# Rust for microbitv2

The [microbit](https://github.com/nrf-rs/microbit) has a built in bootloader

## Prerequisites

Embassy requires a specific version of nightly. You won't have to do anything though as the rust-toolchain file will use the correct versions when you build.

* Install any dependencies and [probe-run](https://github.com/knurling-rs/probe-run#installation) which provides `cargo run` functionality for microcontrollers
* On linux you need the following udev rules saved to somewhere like /etc/udev/rules.d/50-cmsis-dap.rules and then reload your udev rules with something like `sudo udevadm control -R`

```bash
# 0d28:0204 DAPLink
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
```

## Flashing

Optionally, when you want to flash the chip so it can work stand alone we use [cargo-flash](https://github.com/probe-rs/cargo-flash#prerequisites) with `cargo flash --release --chip nRF52833_xxAA`

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
