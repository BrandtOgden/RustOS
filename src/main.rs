#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut writer = vga::Writer::new();
    writer.write("Hello World!\nThis is pretty cool\nshort");
    // writer.write("Hello");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
