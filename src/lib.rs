#![no_std]

pub fn greetings() {
    let msg = b"Debug: Write syscall called\n";
    let mut _ret: usize;

    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1usize,
            in("rdi") 1usize, // stderr
            in("rsi") msg.as_ptr(),
            in("rdx") msg.len(),
            out("rcx") _,
            out("r11") _,
            lateout("rax") _ret,
        );
    }
}

pub fn info() {
    print::print("Template is working!");
}
