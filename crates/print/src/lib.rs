#![no_std]

pub fn print_crate(msg: &str) {
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

static CHAR: &[u8] = b"Test 4: Static crate printing\n";

#[inline(never)]
pub extern "C" fn print_crate_static() {
    for c in CHAR {
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
