fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=crates/");
    
    // Static linking flags
    println!("cargo:rustc-link-arg=-static");
    println!("cargo:rustc-link-arg=--no-dynamic-linker");
    println!("cargo:rustc-link-arg=-n");
    
    // Disable position independent code
    println!("cargo:rustc-link-arg=--no-pie");
    
    // Compile assembly startup code
    cc::Build::new()
        .file("src/start.s")
        .flag("-fno-pic")
        .flag("-fno-pie")
        .compile("start");
}
