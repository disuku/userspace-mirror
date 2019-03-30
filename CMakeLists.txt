cmake_minimum_required(VERSION 3.13)
add_library(rust-kmod-hello-world OBJECT src-server/buse.c src-server/busexmp.c src-server/loopback.c)