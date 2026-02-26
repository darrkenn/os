#!/bin/bash

cd kernel
cargo bootimage --$1
cd ../

qemu-system-x86_64 -drive format=raw,file=target/x86_64-os/$1/bootimage-kernel.bin
