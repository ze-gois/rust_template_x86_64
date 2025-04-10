fn main() {
    println!("cargo:rerun-if-changed=src/start.s");
    println!("cargo:rerun-if-changed=linker.ld");
    cc::Build::new().file("src/start.s").compile("start");
    // println!("cargo:rerun-if-changed=build.rs");
    // println!("cargo:rerun-if-changed=src/");
    // println!("cargo:rerun-if-changed=crates/");

    // // Linker configuration
    // // println!("cargo:rustc-link-arg=-nostartfiles");
    // println!("cargo:rustc-link-arg=-static");
}
