const MAILBOX_BASE: u32 = 0x2000B880;

#[allow(dead_code)]
pub enum Channel {
    PowerManagement = 0,
    Framebuffer = 1,
    VirtualUart = 2,
    Vchiq = 3,
    Leds = 4,
    Buttons = 5,
    Touchscreen = 6,
    Unused = 7,
    TagsArmToVc = 8,
    TagsVcToArm = 9,
}

const MAILBOX_FULL: u32 = 1 << 31;
const MAILBOX_EMPTY: u32 = 1 << 30;

#[allow(dead_code)]
struct Mailbox {
    read: u32,
    padding: [u32; 3],
    peek: u32,
    sender: u32,
    status: u32,
    configuration: u32,
    write: u32,
}

pub fn write(channel: Channel, addr: u32) {
    // addr must be a multiple of 16.
    if addr & 0xFu32 != 0 { return };

    let mailbox = MAILBOX_BASE as *mut Mailbox;
    unsafe {
        // TODO Can I do this better?
        while (*mailbox).status & MAILBOX_FULL != 0 {
            asm!("");
        }

        (*mailbox).write = addr + (channel as u32);
    }
}

pub fn read(channel: Channel) -> bool {
    let mailbox = MAILBOX_BASE as *mut Mailbox;
    unsafe {
        while (*mailbox).status & MAILBOX_EMPTY != 0 {
            asm!("");
        }

        let ra = (*mailbox).read;
        if (ra & 0xF) == (channel as u32) {
            return (ra >> 4) == 0;
        }
    }

    return false;
}
