#![no_std]

static CHAR: u8 = b'X';

#[inline(never)]
pub extern "C" fn minimal_print_static() {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1usize,
            in("rdi") 1usize,
            in("rsi") &CHAR,    // Use static instead of stack variable
            in("rdx") 1usize,
            out("rcx") _,
            out("r11") _,
            lateout("rax") _,
        );
    }
}

// Add to crates/print/src/lib.rs
#[inline(never)]
pub extern "C" fn no_memory_access() {
    // This function just performs some arithmetic
    let x = 42;
    let _y = x + 10;
    // No memory access beyond registers
}

#[inline(never)]
pub extern "C" fn minimal_with_frame() {
    unsafe {
        core::arch::asm!(
            // Set up our own stack frame
            "push rbp",
            "mov rbp, rsp",
            "sub rsp, 16", // Reserve space
            // Do the syscall with direct character
            "mov rax, 1",        // write syscall
            "mov rdi, 1",        // stdout
            "lea rsi, [rip+8f]", // pointer to data (no stack reference)
            "mov rdx, 1",        // length
            "syscall",
            // Clean up and return
            "mov rsp, rbp",
            "pop rbp",
            "ret",
            "8:",
            ".byte 'Y'", // Character directly in assembly
        );
    }
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn minimal_print_c() {
    let c = b'X';

    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1usize,
            in("rdi") 1usize,
            in("rsi") &c,
            in("rdx") 1usize,
            out("rcx") _,
            out("r11") _,
            lateout("rax") _,
        );
    }
}

#[inline(never)]
pub extern "C" fn minimal_print() {
    // Just print a single character using a simpler approach
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1usize,
            in("rdi") 1usize,
            in("rsi") &CHAR,
            in("rdx") 1usize,
            out("rcx") _,
            out("r11") _,
            lateout("rax") _,
        );
    }
}

// Minimal test that uses fixed assembly
#[inline(never)]
pub extern "C" fn minimal_test() {
    unsafe {
        // Just output a single character without any variables
        core::arch::asm!(
            "mov rax, 1",        // write syscall
            "mov rdi, 1",        // stdout fd
            "lea rsi, [rip+9f]", // Get char address relative to instruction pointer
            "mov rdx, 1",        // length 1
            "syscall",
            "9:",
            ".ascii \"X\"",
            options(noreturn)
        );
    }
}

// Function with explicit register preservation
#[inline(never)]
pub extern "C" fn register_safe_test() {
    unsafe {
        core::arch::asm!(
            // Save ALL caller-saved registers
            "push rax",
            "push rcx",
            "push rdx",
            "push rsi",
            "push rdi",
            "push r8",
            "push r9",
            "push r10",
            "push r11",
            // Syscall to write "Z" to stdout
            "mov rax, 1",
            "mov rdi, 1",
            "lea rsi, [rip+8f]",
            "mov rdx, 1",
            "syscall",
            // Restore registers in reverse order
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rdi",
            "pop rsi",
            "pop rdx",
            "pop rcx",
            "pop rax",
            "ret",
            "8:",
            ".ascii \"Z\"",
        );
    }
}

// Simple function that just returns a value without I/O
#[inline(never)]
pub extern "C" fn return_forty_two() -> u64 {
    42
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn print(msg: &str) {
    // Check stack alignment first
    unsafe {
        let rsp: usize;
        core::arch::asm!("mov {}, rsp", out(reg) rsp);

        let debug_msg = if rsp % 16 == 0 {
            "DEBUG: Stack is 16-byte aligned in print module\n"
        } else if rsp % 16 == 8 {
            "DEBUG: Stack is 8-byte aligned in print module\n"
        } else {
            "DEBUG: Stack has unusual alignment in print module\n"
        };

        // Print debug message with direct syscall
        let bytes = debug_msg.as_bytes();
        core::arch::asm!(
            "syscall",
            in("rax") 1usize,
            in("rdi") 2usize,   // stderr
            in("rsi") bytes.as_ptr(),
            in("rdx") bytes.len(),
            out("rcx") _,
            out("r11") _,
            lateout("rax") _,
        );

        // Now try to print the actual message
        let bytes = msg.as_bytes();
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

#[inline(never)]
pub extern "C" fn simple_test() {
    // Just print a fixed message, no parameters
    unsafe {
        let msg = "Simple test function works!\n";
        let bytes = msg.as_bytes();

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

#[inline(never)]
pub extern "C" fn minimal_test2() {
    unsafe {
        // Just output a single character without any variables
        core::arch::asm!(
            "mov rax, 1",        // write syscall
            "mov rdi, 1",        // stdout fd
            "lea rsi, [rip+7f]", // Get char address relative to instruction pointer
            "mov rdx, 1",        // length 1
            "syscall",
            "7:",
            ".ascii \"X\"",
            options(noreturn)
        );
    }
}

#[inline(never)]
pub extern "C" fn pure_asm_test() {
    unsafe {
        core::arch::asm!(
            // Full function in assembly with no references to Rust variables
            "push rbp",
            "mov rbp, rsp",
            "sub rsp, 16",
            "mov rax, 1",         // write syscall
            "mov rdi, 1",         // stdout
            "lea rsi, [rip+99f]", // direct string reference
            "mov rdx, 5",         // length
            "syscall",
            "mov rsp, rbp",
            "pop rbp",
            "ret",
            "99:",
            ".ascii \"HELLO\"",
            options(noreturn)
        );
    }
}
