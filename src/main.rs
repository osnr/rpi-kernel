#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]

mod gpio;
mod timer;

#[no_mangle]
pub extern fn main() {
    loop {
        gpio::write(gpio::Pin::Act, true);
        timer::sleep(5000000);
        gpio::write(gpio::Pin::Act, false);
        timer::sleep(5000000);
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() {}
