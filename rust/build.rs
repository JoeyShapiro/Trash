fn main() {
   println!("cargo:rerun-if-changed=src/asm/sys.s");
   cc::Build::new()
       .file("src/asm/sys.s")
       .compile("sys");
}
