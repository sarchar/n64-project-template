use std::env;

fn main() -> () {
    let n64_inst = env::var("N64_INST").expect("N64_INST must be set to build this project");

    println!("cargo:rustc-link-search=native={n64_inst}/mips64-libdragon-elf/lib");
    println!("cargo:rustc-link-lib=static=dragon");
    println!("cargo:rustc-link-lib=static=dragonsys");

    println!("cargo:rustc-link-search=native={n64_inst}/lib/gcc/mips64-libdragon-elf/13.2.0");
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=g");
    println!("cargo:rustc-link-lib=static=nosys");
    println!("cargo:rustc-link-lib=static=gcc");
}
