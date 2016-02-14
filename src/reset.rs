use core::intrinsics::volatile_store;

use timer;

const PM_RSTC: u32 = 0x2010001c;
const PM_WDOG: u32 = 0x20100024;
const PM_PASSWORD: u32 = 0x5a000000;
const PM_RSTC_WRCFG_FULL_RESET: u32 = 0x00000020;

pub fn reset() {
    timer::sleep(100000);

    // timeout = 1/16th of a second? (whatever)
    unsafe {
        volatile_store(PM_WDOG as *mut u32, PM_PASSWORD | 1);
        volatile_store(PM_RSTC as *mut u32, PM_PASSWORD | PM_RSTC_WRCFG_FULL_RESET);
    }

    loop { unsafe { asm!(""); } }
}
