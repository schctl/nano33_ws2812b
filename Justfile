# Recipes to run the Arduino Nano 33 demo

build profile='dev':
    cargo build   -p nano33demo --profile {{profile}}
    cargo objcopy -p nano33demo --profile {{profile}} -- -O binary target/driver.bin

upload:
    bossac -U -i -e -w -b target/driver.bin -R
