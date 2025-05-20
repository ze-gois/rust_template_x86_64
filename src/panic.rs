use core::panic::PanicInfo;

use super::_print as print1;
use super::mamod::print as print0;
use print::print as print2;
use template::print as print3;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    print0("0, ");
    print1("1, ");
    print2("2, ");
    print3("3");

    let mut count = 5;
    loop {
        count -= 1;
        print0(".");
        if count == 0 {
            template::exit();
        }
    }
}
