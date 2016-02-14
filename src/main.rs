#![feature(lang_items, asm, repr_simd, core_intrinsics)]
#![no_std]

pub mod interrupts;

mod ringbuf;
mod keyboard;
mod gpio;
mod timer;
#[macro_use] mod uart;
mod reset;

mod console;

use core::fmt::Arguments;

mod gl;

#[no_mangle]
pub extern fn main() {
    timer::sleep(500000);

    uart::init();
    keyboard::init();
    gl::init();
    
    interrupts::enable();

    let mut console = console::Console { row: 0, col: 0 };
    console.run();
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

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt(fmt: Arguments, file_line: &(&'static str, u32)) {
    loop {
        use core::fmt::Write;
        uart::get_uart().write_fmt(fmt);

        timer::sleep(5000000);
        reset::reset();
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
#[no_mangle]
pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32
        }
        i += 1;
    }
    return 0;
}
