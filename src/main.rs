#![feature(lang_items, asm, repr_simd, core_intrinsics)]
#![no_std]

pub mod interrupts;

mod ringbuf;
mod keyboard;
mod gpio;
mod timer;

#[macro_use]
mod uart;

use core::intrinsics::volatile_store;

use core::fmt::Arguments;

mod gl;

#[no_mangle]
pub extern fn main() {
    timer::sleep(500000);

    uart::init();
    keyboard::init();
    gl::init();
    
    interrupts::enable();

    gl::put_str("hello", 0, 0);

    let mut x = 0;
    loop {
        let wut = gpio::read(gpio::Pin::TwentyFour);

        let c = uart::getc();
        if c == '`' as u8 {
            reset();
        } else {
            gl::put_char(c, x, 13);
            
            x += 8;
            uart::putc(c);
            println!("\nclock says: {}", wut);
        }

        // let c = keyboard::wait_for_char();
        // println!("{:x}", c);
    }
}

const RPI_VECTOR_START: u32 = 0x0;

#[no_mangle]
pub extern fn prologue(table_start: isize, table_end: isize) {
    let vector: *mut u32 = RPI_VECTOR_START as *mut u32;

    let mut table = table_start;
    while table < table_end {
        let there = unsafe { vector.offset((table - table_start) / 4) };
        let here = table as *mut u32;

        unsafe { *there = *here; }

        table += 4;
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
#[lang = "panic_fmt"] extern fn panic_fmt(fmt: Arguments, file_line: &(&'static str, u32)) {
    loop {
        uart::get_uart().write_fmt(fmt);
        gpio::write(gpio::Pin::Rx, true);
        println!("at {}:{}", file_line.0, file_line.1);
        gpio::write(gpio::Pin::Rx, false);
    }
}
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
#[no_mangle]
pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    return s;
}
