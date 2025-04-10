#![no_std]
#![no_main]

mod panic;

use print::*;
use x86_64_template::*;

#[unsafe(no_mangle)]
pub extern "C" fn entry(_stack_pointer: *mut u64) -> ! {
    // Test 1: Direct inline assembly
    print_direct("Test 1: Direct inline assembly works\n");

    print_direct("About to call pure asm test...\n");
    pure_asm_test();

    // Try with static variable
    print_direct("About to call minimal_print_static...\n");
    minimal_print_static();

    // Try with extern C function
    print_direct("About to call minimal_print_c...\n");
    minimal_print_c();

    print_direct("About to perform minimal character writing...\n");
    minimal_print();

    print_direct("About to perform register safe test...\n");
    register_safe_test();

    print_direct("About to call simple_test...\n");
    simple_test();

    // Test 2: Call to external function
    print_direct("About to call print...\n");
    print("Test 2: Called print::print successfully!\n");

    // If we get here, it worked!
    print_direct("All tests passed! Entering infinite loop.\n");

    info();
    loop {}
}

// Helper function using direct inline assembly
fn print_direct(msg: &str) {
    let bytes = msg.as_bytes();
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1usize,    // write syscall
            in("rdi") 1usize,    // stdout
            in("rsi") bytes.as_ptr(),
            in("rdx") bytes.len(),
            out("rcx") _,
            out("r11") _,
            lateout("rax") _,
        );
    }
}
