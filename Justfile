build chip='nrf52840' profile='dev':
    cargo build   --profile {{profile}} --features {{chip}} --target $(./scripts/targets.sh {{chip}})
    cargo objcopy --profile {{profile}} --features {{chip}} --target $(./scripts/targets.sh {{chip}}) -- -O binary target/driver.bin

upload:
    bossac -U -i -e -w -b target/driver.bin -R
