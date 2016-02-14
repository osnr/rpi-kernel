use core::slice;

use core::intrinsics::{volatile_store};

mod mailbox;
pub mod font;

const GPU_NOCACHE: u32 = 0x40000000;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;

// The simd is a giant alignment hack.
#[allow(dead_code)]
#[repr(C, simd)]
struct FbConfig {
    width: u32,
    height: u32,

    virtual_width: u32,
    virtual_height: u32,

    pitch: u32,
    depth: u32,

    x_offset: u32,
    y_offset: u32,

    framebuffer: u32,

    size: u32,

    _align1: u32,
    _align2: u32,
}

static mut fb_config: FbConfig = FbConfig {
    width: WIDTH as u32,
    height: HEIGHT as u32,

    virtual_width: WIDTH as u32,
    virtual_height: HEIGHT as u32,

    depth: 32,
    x_offset: 0,
    y_offset: 0,

    // The GPU will fill these in.
    pitch: 0,
    framebuffer: 0,
    size: 0,

    _align1: 0,
    _align2: 0,
};

pub fn init() {
    let uncached_fb_config_addr = unsafe { (&mut fb_config as *mut FbConfig as u32) + GPU_NOCACHE };

    mailbox::write(mailbox::Channel::Framebuffer, uncached_fb_config_addr);
    mailbox::read(mailbox::Channel::Framebuffer);
}

fn get_fb() -> &'static mut [u32] {
    return unsafe { slice::from_raw_parts_mut(fb_config.framebuffer as *mut u32, (fb_config.size / 4) as usize) };
}

fn get_width() -> usize {
    return unsafe { fb_config.width as usize };
}

pub fn put_pixel(color: u32, x: usize, y: usize) {
    let pixel: *mut u32 = &mut get_fb()[(y * get_width()) + x] as *mut u32;
    unsafe { volatile_store(pixel, color); }
}

pub fn put_char(c: u8, x: usize, y: usize) {
    let glyph: [u8; 13] = font::FONT[c as usize - 32];

    for row in 0..font::HEIGHT {
        for col in 0..font::WIDTH {
            let pixel_bw = (glyph[row] >> col) & 1;

            let color =
                if pixel_bw == 0 {
                    0xFF000000
                } else {
                    0xFFFFFFFF
                };

            put_pixel(color, x + 8 - col, y + 12 - row);
        }
    }
}

pub fn put_str(s: &str, x: usize, y: usize) {
    let mut new_x = x;

    for c in s.as_bytes() {
        put_char(*c, new_x, y);
        new_x += 8;
    }
}
