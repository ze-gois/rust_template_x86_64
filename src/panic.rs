use core::panic::PanicInfo;

use super::_print as print0;
use super::mamod::print as print1;
use print::print as print3;
use template::print as print2;
use template::print as print4;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    print0("0, ");
    print1("1, ");
    print2("2, ");
    print3("3, ");
    print4("4");

    let mut count = 5;
    loop {
        count -= 1;
        print0(".");
        if count == 0 {
            template::exit();
        }
    }
}
