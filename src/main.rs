#![no_std]
#![no_main]

mod mamod;
mod panic;

use print;
use template;

fn _print(msg: &str) {
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
    _print("Test 0: Local inline assembly works\n");
    mamod::print("Test 1: Local module inline assembly works\n");
    template::print("Test 2: Lib inline assembly works\n");
    print::print("Test 3: Crate inline assembly works\n");
    print::print_static();
    panic!("Test 5: Panic works");
}
