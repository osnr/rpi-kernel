use keyboard;

pub fn enable() {
    unsafe {
        asm!("mrs r0, cpsr");
        asm!("bic r0, r0, #0x80"); 
        asm!("msr cpsr_c, r0");
    }
}

#[no_mangle] pub unsafe extern fn interrupt_vector(_pc: u32) {
    keyboard::interrupt();
}

#[no_mangle] pub unsafe extern fn fast_interrupt_vector(_pc: u32) {}
#[no_mangle] pub unsafe extern fn software_interrupt_vector(_pc: u32) {}

#[no_mangle] pub unsafe extern fn reset_vector(_pc: u32) {}
#[no_mangle] pub unsafe extern fn undefined_instruction_vector(_pc: u32) {}
#[no_mangle] pub unsafe extern fn prefetch_abort_vector(_pc: u32) {}
#[no_mangle] pub unsafe extern fn data_abort_vector(_pc: u32) {}
