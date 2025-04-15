use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    print::print_crate("Panicking.");
    loop {}
}
