#![no_std]

pub fn print(msg: &str) {
    // Print entire string at once using write syscall
    let bytes = msg.as_bytes();
    let ptr = bytes.as_ptr();
    let len = bytes.len();
    unsafe {
        core::arch::asm!(
            "mov rax, 1",      // write syscall
            "mov rdi, 1",      // stdout fd
            "mov rsi, {0}",    // pointer to string
            "mov rdx, {1}",    // length of string
            "syscall",
            in(reg) ptr,
            in(reg) len,
            out("rax") _,
            out("rcx") _,
            out("r11") _,
            out("rdi") _,
            out("rsi") _,
            out("rdx") _,
        );
    }
}

static MESSAGE: &[u8] = b"Test 4: Static crate printing\n";

#[inline(never)]
pub extern "C" fn print_static() {
    for c in MESSAGE {
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rax") 1usize,
                in("rdi") 1usize,
                in("rsi") c,
                in("rdx") 1usize,
                out("rcx") _,
                out("r11") _,
                lateout("rax") _,
            );
        }
    }
}
