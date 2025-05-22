#![no_std]
#![no_main]
pub mod amod;
mod panic;

use print;
use template;

pub struct Writer;

// Simple print function using syscall
#[inline(always)]
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
            options(nostack, preserves_flags, readonly)
        );
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        _print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = Writer;
        let _ = write!(&mut writer, $($arg)*);
    }};
}

#[unsafe(no_mangle)]
pub extern "C" fn entry(_stack_pointer: *mut u64) -> ! {
    // Run the tests
    _print("Test 0: src/main.rs\n");
    amod::print("Test 1: src/amod.rs\n");
    template::print("Test 2: src/lib.rs\n");
    print::print("Test 3: crates/print/src/lib.rs\n");
    print::print_static();

    print::info!("Test {}: {}\n", 5, "crate macro");
    template::info!("Test {}: {}\n", 6, "lib macro");
    info!("Test {}: {}\n", 7, "binary macro");
    // Trigger panic handler
    panic!("Test 8: src/panic.rs");
}
