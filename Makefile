OBJCOPY=arm-none-eabi-objcopy
OBJDUMP=arm-none-eabi-objdump

TARGET=arm-unknown-linux-gnueabihf

# Files
OUT_DIR=target/$(TARGET)/debug
NAME=kernel

.PHONY: build clean listing $(OUT_FILE)

all: build listing
build: $(OUT_DIR)/$(NAME).bin
listing: $(OUT_DIR)/$(NAME).list

$(OUT_DIR)/lib$(NAME).a:
	cargo rustc --target=arm-unknown-linux-gnueabihf --verbose

$(OUT_DIR)/%.bin: $(OUT_DIR)/lib%.a
	$(OBJCOPY) -O binary $< $@

$(OUT_DIR)/%.list: $(OUT_DIR)/lib%.a
	$(OBJDUMP) -D $< > $@

clean:
	cargo clean
