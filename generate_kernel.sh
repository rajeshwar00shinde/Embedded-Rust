cargo rustc -- -C link-arg=--script=./linker.ld

arm-none-eabi-objcopy -O binary kernel kernel8.img
