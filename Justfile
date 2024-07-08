build profile='dev':
    cargo build   --profile {{profile}}
    cargo objcopy --profile {{profile}} -- -O binary target/nano33_baremetal.bin

upload:
    bossac -U -i -e -w -b target/nano33_baremetal.bin -R
