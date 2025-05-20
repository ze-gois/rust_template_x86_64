use core::panic::PanicInfo;

use super::mamod::print as print0;
use print::print as print1;
use template::print as print2;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    print0("Panicking 0;");
    print1("Panicking 1;");
    print2("Panicking 2;");

    let mut count = 5;
    loop {
        count -= 1;
        print0(".");
        if count == 0 {
            template::exit();
        }
    }
}
