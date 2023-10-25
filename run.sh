#! /bin/bash

sudo qemu-system-x86_64 \
    -L target/x86_64 \
    -bios usr/share/edk2.git/ovmf-x64/OVMF-pure-efi.fd \
    -hda fat:rw:dist/x86_64
