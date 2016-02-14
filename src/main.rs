#![feature(lang_items, asm, repr_simd, core_intrinsics)]
#![no_std]

mod gpio;
mod timer;

#[macro_use]
mod uart;

use core::intrinsics::{volatile_load, volatile_store};

mod gl;

#[no_mangle]
pub extern fn main() {
    timer::sleep(500000);

    uart::init();

    gl::init();

    gl::put_str("hello", 0, 0);

    // let mut prev_pin = false;
    loop {
        // prev_pin = !prev_pin;
        // gpio::write(gpio::Pin::Rx, prev_pin); // Same as Act LED.

        let c = uart::getc();
        if c == 'r' as u8 {
            reset();
        } else {
            uart::putc(c);
        }
    }
}

const PM_RSTC: u32 = 0x2010001c;
const PM_WDOG: u32 = 0x20100024;
const PM_PASSWORD: u32 = 0x5a000000;
const PM_RSTC_WRCFG_FULL_RESET: u32 = 0x00000020;

fn reset() {
    timer::sleep(100000);

    // timeout = 1/16th of a second? (whatever)
    unsafe {
        volatile_store(PM_WDOG as *mut u32, PM_PASSWORD | 1);
        volatile_store(PM_RSTC as *mut u32, PM_PASSWORD | PM_RSTC_WRCFG_FULL_RESET);
    }

    loop { unsafe { asm!(""); } }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() {}
#[no_mangle] pub extern fn __aeabi_unwind_cpp_pr0() {}
#[no_mangle] pub extern fn __aeabi_unwind_cpp_pr1() {}

#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8,
                            n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dest;
}
