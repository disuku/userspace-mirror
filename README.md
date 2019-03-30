# Userspace Mirror

So thankful for: https://github.com/acozzette/BUSE

## Usage

    modprobe nbd
    ./build/busexmp /dev/nbd0 <src file>

Then you can use /dev/nbd0 as a regular block device:

    mkfs.ext4 /dev/nbd0