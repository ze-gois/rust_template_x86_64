use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    print::print("Panicking:");
    let mut count = 5;
    loop {
        count -= 1;
        print::print(".");
        if count == 0 {
            template::exit();
        }
    }
}
