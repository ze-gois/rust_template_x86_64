#![no_std]
#![no_main]

pub mod mamod;
mod panic;

use print;
use template;

fn _print(msg: &str) {
    let bytes = msg.as_bytes();
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") 1usize => _,
            in("rdi") 1usize,
            in("rsi") bytes.as_ptr(),
            in("rdx") bytes.len(),
            out("rcx") _,
            out("r11") _,
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn entry(_stack_pointer: *mut u64) -> ! {
    _print("Test 0: src/main.rs\n");
    mamod::print("Test 1: src/mamod.rs\n");
    template::print("Test 2: src/lib.rs\n");
    print::print("Test 3: crates/print/src/lib.rs\n");
    print::print_static();
    panic!("Test 5: src/panic.rs");
}
