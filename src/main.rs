#![feature(lang_items, asm, repr_simd)]
#![no_std]

mod gl;
mod gpio;
mod timer;

#[macro_use]
mod uart;
use core::fmt::Write;

#[no_mangle]
pub extern fn main() {
    timer::sleep(50000);

    uart::init();
    loop {
        println!("hello {}", 300);
        timer::sleep(5000000);
    }

    gl::init();

    gl::set_pixel(0xFFFFFFFF, 0, 0);
    gl::set_pixel(0xFFFFFFFF, 0, 1);
    gl::set_pixel(0xFF00FFFF, 0, 2);
    gl::set_pixel(0xFFFF00FF, 0, 3);
    gl::set_pixel(0xFFFFFFFF, 4, 0);

    loop {
        gpio::write(gpio::Pin::Rx, true); // Same as Act LED.
        timer::sleep(5000000);
        gpio::write(gpio::Pin::Rx, false);
        timer::sleep(5000000);
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() {}
#[no_mangle] pub extern fn __aeabi_unwind_cpp_pr0() {}
