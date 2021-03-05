#!/bin/bash
EDK2=~/edk2
cp target/x86_64-unknown-uefi/debug/uefi-chainloader.efi qemu/hda-contents/ldr.efi && qemu-system-x86_64 \
-s \
-serial file:serial.log \
--pflash $EDK2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF.fd \
-boot d \
-drive format=raw,file=fat:rw:qemu/hda-contents \
-cdrom qemu/UefiShell.iso \
-smp 2 \
-m 1024 \
