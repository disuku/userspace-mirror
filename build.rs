// build.rs

extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/buse.c")
        .file("src/buse-shim.c")
        .compile("buse");
}