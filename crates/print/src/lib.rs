#![no_std]

pub struct Writer;

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

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = $crate::Writer;
        let _ = write!(&mut writer, $($arg)*);
    }};
}

static MESSAGE: &[u8] = b"Test 4: crates/print/src/lib.rs (static)\n";

#[inline(never)]
pub extern "C" fn print_static() {
    for c in MESSAGE {
        unsafe {
            core::arch::asm!(
                "syscall",
                inlateout("rax") 1usize => _,
                in("rdi") 1usize,
                in("rsi") c,
                in("rdx") 1usize,
                out("rcx") _,
                out("r11") _,
                options(nostack, preserves_flags, readonly)
            );
        }
    }
}
