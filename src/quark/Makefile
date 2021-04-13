run-qemu:
	@qemu-system-riscv64 -machine virt -nographic -bios ../bootloader/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/quark.bin,addr=0x80200000

run-k210:

build:
	@cargo build --release
	@rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/quark --strip-all -O binary target/riscv64gc-unknown-none-elf/release/quark.bin

solve-filelock:
	@rm ~/.cargo/.package-cache