fn main() {
    cc::Build::new()
        .file("src/asm/sys.s")
        .compile("sys");
}