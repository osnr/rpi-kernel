pub fn sleep(value: u32) {
    for _ in 1..value {
        unsafe { asm!(""); }
    }
}
