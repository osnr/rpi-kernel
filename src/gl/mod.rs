#[repr(simd)]

use core::slice;

mod mailbox;

const GPU_NOCACHE: u32 = 0x40000000;

const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;

struct SixteenBytes(u64, u64);
#[allow(dead_code)]
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
    _align: [SixteenBytes; 0],
}

static mut fb_config: FbConfig = FbConfig {
    width: WIDTH,
    height: HEIGHT,

    virtual_width: WIDTH,
    virtual_height: HEIGHT,

    depth: 32,
    x_offset: 0,
    y_offset: 0,

    // The GPU will fill these in.
    pitch: 0,
    framebuffer: 0,
    size: 0,

    _align: [],
};

pub fn init() {
    unsafe {
        let uncached_fb_config_addr = (&mut fb_config as *mut FbConfig as u32) + GPU_NOCACHE;

        mailbox::write(mailbox::Channel::Framebuffer, uncached_fb_config_addr);
        mailbox::read(mailbox::Channel::Framebuffer);
    }
}

unsafe fn get_fb() -> &'static mut [u32] {
    return slice::from_raw_parts_mut(fb_config.framebuffer as *mut u32, (fb_config.size / 4) as usize);
}

pub fn set_pixel(color: u32, x: usize, y: usize) {
    unsafe {
        get_fb()[(y * fb_config.width as usize) + x] = color;
    }
}
