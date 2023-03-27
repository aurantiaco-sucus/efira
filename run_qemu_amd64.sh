#!/bin/bash
cargo build --target x86_64-unknown-uefi
rm -rf /tmp/efira-esp
mkdir -p /tmp/efira-esp/efi/boot
cp target/x86_64-unknown-uefi/debug/efira.efi /tmp/efira-esp/efi/boot/bootx64.efi
qemu-system-x86_64 \
 -drive if=pflash,format=raw,readonly=on,file=ovmf/OVMF_CODE.fd \
 -drive if=pflash,format=raw,file=ovmf/OVMF_VARS.fd \
 -drive if=ide,format=raw,file=fat:rw:/tmp/efira-esp \
 -serial stdio
rm -rf /tmp/efira-esp
