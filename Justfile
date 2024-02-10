default:
	@echo Nothing to do

build:
    cargo build --release
    @"$N64_INST/bin/mips64-libdragon-elf-objcopy" -O binary ./target/mips-nintendo64-none/release/n64-project-template \
            ./target/mips-nintendo64-none/release/n64-project-template.bin 
    @"$N64_INST/bin/n64tool" -t "TEMPLATE" -h "$N64_INST/mips64-libdragon-elf/lib/header" \
            -o ./target/mips-nintendo64-none/release/n64-project-template.z64 -l 1M \
            ./target/mips-nintendo64-none/release/n64-project-template.bin
    @"$N64_INST/bin/chksum64" ./target/mips-nintendo64-none/release/n64-project-template.z64
    @echo
    @echo "ROM is in ./target/mips-nintendo64-none/release/n64-project-template.z64"
