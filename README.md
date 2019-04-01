# Userspace Mirror

So thankful for: https://github.com/acozzette/BUSE

## Building

    cargo build [--release]

## Usage

    modprobe nbd
    ./target/release/userspace-mirror /dev/nbd0 <src file>

Then you can use /dev/nbd0 as a regular block device:

    mkfs.ext4 /dev/nbd0
    mount /dev/nbd0 /mnt