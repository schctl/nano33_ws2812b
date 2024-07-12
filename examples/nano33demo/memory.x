MEMORY
{
    # the default arduino bootloader sits here and occupies 64K
    # so we don't want to try to overwrite that
    FLASH : ORIGIN = 0x00010000, LENGTH = 1024K - 0x10000
    RAM : ORIGIN = 0x20000000, LENGTH = 256K
}
