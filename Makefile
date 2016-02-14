OBJCOPY=arm-none-eabi-objcopy
OBJDUMP=arm-none-eabi-objdump

TARGET=arm-unknown-linux-gnueabihf

LIBCORE=

# Files
NAME=kernel

.PHONY: build clean listing $(OUT_FILE)

all: kernel.img kernel.list

kernel.img: kernel.elf
	$(OBJCOPY) kernel.elf -O binary kernel.img

kernel.list: kernel.img
	$(OBJDUMP) -d kernel.elf > kernel.list

kernel.elf: src/start.o src/main.o
	arm-none-eabi-gcc -O0 -Wl,-gc-sections -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostdlib $^ -o $@

%.o: %.rs
	rustc --target arm-unknown-linux-gnueabihf -O -L /Users/osnr/dev --crate-type="staticlib" $< -o $@

%.o: %.s
	arm-none-eabi-as $< -o $@

install: kernel.img
	rpi-install.py kernel.img

clean:
	rm -f kernel.img
	rm -f kernel.elf
	rm src/*.o
