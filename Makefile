arch ?= x86_64
target := $(arch)-unknown-rose
kernel := build/kernel-rose-$(arch).bin
iso := build/rose-$(arch).iso
roselib := target/$(target)/debug/librose.a

arch_dir := src/arch/$(arch)/boot
targets_dir := targets

linker_script := $(arch_dir)/linker.ld
grub_cfg := $(arch_dir)/grub/grub.cfg
asm_sources := $(wildcard $(arch_dir)/*.asm)
asm_objects:= $(patsubst $(arch_dir)/%.asm, build/arch/$(arch)/%.o, $(asm_sources))

.PHONY: all clean run iso

all: $(kernel)

clean:
	@rm -r build

run: $(iso)
	@qemu-system-$(arch) -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/iso/boot/grub
	@cp $(kernel) build/iso/boot/kernel.bin
	@cp $(grub_cfg) build/iso/boot/grub/grub.cfg
	@grub-mkrescue -o $(iso) build/iso 2> /dev/null
	@rm -r build/iso

$(kernel): kernel $(roselib) $(asm_objects) $(linker_script)
	@ld -n -T $(linker_script) -o $(kernel) $(asm_objects) $(roselib)

kernel:
	@RUST_TARGET_PATH=$(shell pwd)/targets /home/marek/.cargo/bin/cargo build --target $(target)

build/arch/$(arch)/%.o: $(arch_dir)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@
