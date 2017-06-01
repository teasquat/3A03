default: build

build: target/kernel.bin

.PHONY: clean

target/multiboot_header.o: src/asm/multiboot_header.asm
	mkdir -p target
	nasm -f elf64 src/asm/multiboot_header.asm -o target/multiboot_header.o

target/boot.o: src/asm/boot.asm
	mkdir -p target
	nasm -f elf64 src/asm/boot.asm -o target/boot.o


target/long_mode_init.o: src/asm/long_mode_init.asm
	mkdir -p target
	nasm -f elf64 src/asm/long_mode_init.asm -o target/long_mode_init.o

target/kernel.bin: target/multiboot_header.o target/boot.o target/long_mode_init.o src/asm/linker.ld cargo
	ld -n -o target/kernel.bin -T src/asm/linker.ld target/multiboot_header.o target/boot.o target/long_mode_init.o target/x86_64-unknown-3A03-gnu/release/libfuzz_kernel.a --gc-sections

target/os.iso: target/kernel.bin src/asm/grub.cfg
	mkdir -p target/isofiles/boot/grub
	cp src/asm/grub.cfg target/isofiles/boot/grub
	cp target/kernel.bin target/isofiles/boot/
	grub-mkrescue -o target/os.iso target/isofiles

cargo:
	xargo build --release --target x86_64-unknown-3A03-gnu

run: target/os.iso
	qemu-system-x86_64 -cdrom target/os.iso

clean:
	cargo clean
