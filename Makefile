kernel.img: kernel.elf
	arm-none-eabi-objcopy kernel.elf -O binary kernel.img

kernel.elf: kernel.o
	arm-none-eabi-gcc -O0 -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostartfiles kernel.o -o kernel.elf

%.o: %.rs
	rustc --target arm-unknown-linux-gnueabihf -O --emit=obj $< -o $@

install: kernel.img
	rpi-install.py kernel.img
