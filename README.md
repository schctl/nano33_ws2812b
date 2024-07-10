# Bare Metal NRF52 Demo

This repo contains some code testing bare metal programming on the [Arduino Nano 33 BLE] ([`nrf52840`]).

## Requirements

- Rust Toolchain
- [cargo-binutils](<https://github.com/rust-embedded/cargo-binutils>)
- [arduino/BOSSA](<https://github.com/arduino/BOSSA>)
- [just](<https://github.com/casey/just>) build and upload recipes.

## Build

To build the firmware, use the provided `just build` recipe.

## Upload

> [!IMPORTANT]
> _Ensure the `udev` rules are set before attempting to upload. The [`udev_rules.sh`](./scripts/udev_rules.sh)
> script will do this for you._

Use the `just upload` recipe to call `bossac` and upload the firmware onto the board. Ensure the board
is in bootloader mode before doing so.

[arduino nano 33 ble]: <https://docs.arduino.cc/hardware/nano-33-ble/>
[`nrf52840`]: <https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.8.pdf>
