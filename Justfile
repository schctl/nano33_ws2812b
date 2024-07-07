build:
    cargo build
    cargo objcopy -- -O binary target/nano33_baremetal.bin

