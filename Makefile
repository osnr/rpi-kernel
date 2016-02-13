OBJCOPY=arm-none-eabi-objcopy
OBJDUMP=arm-none-eabi-objdump

TARGET=arm-unknown-linux-gnueabihf

# Files
NAME=kernel

.PHONY: build clean listing $(OUT_FILE)

kernel.img: kernel.elf
	$(OBJCOPY) kernel.elf -O binary kernel.img

kernel.elf: src/main.o
	arm-none-eabi-gcc -O0 -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostartfiles src/main.o -o kernel.elf

%.o: %.rs
	rustc --target arm-unknown-linux-gnueabihf -O --crate-type="lib" --emit=obj $< -o $@

install: kernel.img
	rpi-install.py kernel.img

clean:
	rm kernel.img
	rm kernel.elf
	rm src/*.o
