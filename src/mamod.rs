pub fn print(msg: &str) {
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
