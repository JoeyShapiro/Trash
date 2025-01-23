use std::env;
use std::path::Path;

fn main() {
    let target = env::var("TARGET").unwrap();
    let (arch, os) = target.split_once('-').unwrap();
    
    let asm_path = format!("src/asm/{}-{}-sys.s", arch, os);
    
    if !Path::new(&asm_path).exists() {
        panic!("Missing assembly file for {}", target);
    }
    
    println!("cargo:rerun-if-changed={}", asm_path);
    cc::Build::new()
       .file(asm_path)
       .compile("sys");
}