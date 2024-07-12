# nrf5x WS2812B driver

This repo contains a bare-metal [WS2812B] driver implemented for Nordic `nrf5-` series chips.

# Arduino Nano 33 Sample

Also contained is some sample code running the driver on the Arduino Nano 33 BLE ([nrf52840]), which
is the board I use to test the driver. Read the following sections on how to run this example.

## Requirements

- Rust Toolchain
- [cargo-binutils](<https://github.com/rust-embedded/cargo-binutils>)
- [arduino/BOSSA](<https://github.com/arduino/BOSSA>) with `bossac` in your path
- [just](<https://github.com/casey/just>) for the provided build and upload recipes

## Build

To build the firmware, use the provided `just build` recipe.

## Upload

> [!IMPORTANT]
> _Ensure the `udev` rules are set before attempting to upload. The [`udev_rules.sh`](./examples/nano33demo/scripts/udev_rules.sh)
> script will do this for you._

Use the `just upload` recipe to call `bossac` and upload the firmware onto the board. Ensure the board
is in bootloader mode before doing so.

[arduino nano 33 ble]: <https://docs.arduino.cc/hardware/nano-33-ble/>
[nrf52840]: <https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.9.pdf>
[ws2812b]: <https://cdn-shop.adafruit.com/datasheets/WS2812B.pdf>
