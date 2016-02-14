OBJCOPY=arm-none-eabi-objcopy
OBJDUMP=arm-none-eabi-objdump

TARGET=arm-unknown-linux-gnueabihf

SOURCES := $(shell find src -name '*.rs')

# Files
NAME=kernel

.PHONY: build clean listing $(OUT_FILE)

all: clean kernel.img kernel.list

kernel.img: kernel.elf
	$(OBJCOPY) kernel.elf -O binary kernel.img

kernel.list: kernel.img
	$(OBJDUMP) -d kernel.elf > kernel.list

kernel.elf: src/start.o src/main.o
	arm-none-eabi-gcc -O0 -g -Wl,-gc-sections -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostdlib $^ -o $@

%.o: %.rs $(SOURCES)
	rustc --target arm-unknown-linux-gnueabihf -g -L /Users/osnr/dev --crate-type="staticlib" $< -o $@

%.o: %.s
	arm-none-eabi-as $< -o $@

install: clean kernel.img
	rpi-install.py kernel.img

install-screen: install
	sleep 5
	screen /dev/tty.SLAB_USBtoUART 115200

clean:
	rm -f kernel.img
	rm -f kernel.elf
	rm -f src/*.o
