#![no_std]
#![no_main]

mod panic;

use print::*;
use x86_64_template::*;

fn print_locally(msg: &str) {
    let bytes = msg.as_bytes();
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1usize,
            in("rdi") 1usize,
            in("rsi") bytes.as_ptr(),
            in("rdx") bytes.len(),
            out("rcx") _,
            out("r11") _,
            lateout("rax") _,
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn entry(_stack_pointer: *mut u64) -> ! {
    // Test 1: Direct inline assembly
    print_locally("Test 1: Local inline assembly works\n");
    print_liblly("Test 2: Lib inline assembly works\n");
    print_crate("Test 3: Crate inline assembly works\n");
    print_crate_static();
    panic!("Test 5: Panic works");
}
