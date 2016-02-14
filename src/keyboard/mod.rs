mod ps2;

pub fn init() {
    ps2::init();
}

pub unsafe fn interrupt() { ps2::interrupt(); }

pub fn wait_for_next_scan() -> u8 {
    unsafe {
        while ps2::scanbuf.empty() { asm!(""); }
    }

    return unsafe { ps2::scanbuf.pop() };
}

const SCAN_RELEASE: u8 = 0xF0;
const SCAN_SPECIAL: u8 = 0xE0;

const SCAN_SPECIAL_LEFT: u8 = 0x6B;
const SCAN_SPECIAL_RIGHT: u8 = 0x74;
const SCAN_SPECIAL_UP: u8 = 0x75;
const SCAN_SPECIAL_DOWN: u8 = 0x72;

const SCAN_LSHIFT: u8 = 0x12;
const SCAN_RSHIFT: u8 = 0x59;
const SCAN_CAPSLOCK: u8 = 0x58;

const SCAN_FAIL: u8 = 0xFF;
const READ_FAIL: u8 = 0xFF;

static mut shift: bool = false;
static mut capslock: bool = false;
static mut last_char: Option<char> = None;

pub fn has_char() -> bool {
    unsafe {
        if ps2::scanbuf.empty() { return false; }
        if last_char.is_some() { return true; }
    }

    let scan = unsafe { ps2::scanbuf.pop() };

    match scan {
        SCAN_RELEASE => {
            let scan = wait_for_next_scan();

            if scan == SCAN_LSHIFT || scan == SCAN_RSHIFT {
                unsafe { shift = false; }
            } else if scan == SCAN_SPECIAL {
                wait_for_next_scan(); // Discard next one too.
            }

            return has_char();
        },

        SCAN_SPECIAL => {
            let scan = wait_for_next_scan();

            // TODO: Handle arrow keys here.

            return has_char();
        },

        SCAN_LSHIFT | SCAN_RSHIFT => {
            unsafe { shift = true; }
            return has_char();
        },

        SCAN_CAPSLOCK => {
            unsafe { capslock = !capslock; }
            return has_char();
        },

        _ => {
            unsafe {
                if shift != capslock {
                    last_char = Some(ps2::SHIFT_SCAN_TABLE[scan as usize]);
                } else {
                    last_char = Some(ps2::SCAN_TABLE[scan as usize]);
                }
            }

            return true;
        }
    }
}

pub fn read_char() -> u8 {
    let c = unsafe { last_char.unwrap() };
    unsafe { last_char = None; }

    return c as u8;
}
