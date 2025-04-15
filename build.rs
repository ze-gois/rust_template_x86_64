fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=crates/");
    println!("cargo:rustc-link-arg=-static");
    cc::Build::new().file("src/start.s").compile("start");
}
