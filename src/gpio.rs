use core::intrinsics::{volatile_load, volatile_store};

const GPIO_BASE: u32 = 0x20200000;

// Pin output set: Set an output pin to be 1
// const GPSET0: u32 = 0x2020001C;
// const GPSET1: u32 = 0x20200020;
// Pin output clear: Set an output pin to be 0
// const GPCLR0: u32 = 0x20200028;
// const GPCLR1: u32 = 0x2020002C;
// Pin level: read a pin (high or low)
const GPLEV0: u32 = 0x20200034;
const GPLEV1: u32 = 0x20200038;
// Pin event detect status (has the event occured)
const GPEDS0: u32 = 0x20200040;
const GPEDS1: u32 = 0x20200044;
// Pin rising edge detect 
// const GPREN0: u32 = 0x2020004C;
// const GPREN1: u32 = 0x20200050;
// Pin falling edge detect 
const GPFEN0: u32 = 0x20200058;
const GPFEN1: u32 = 0x2020005C;
// Pin high detect
// const GPHEN0: u32 = 0x20200064;
// const GPHEN1: u32 = 0x20200068;
// Pin low detect
// const GPLEN0: u32 = 0x20200070;
// const GPLEN1: u32 = 0x20200074;
// Pin async rising edge detect
// const GPAREN0: u32 = 0x2020007C;
// const GPAREN1: u32 = 0x20200080;
// Pin async falling edge detect
// const GPAFEN0: u32 = 0x20200088;
// const GPAFEN1: u32 = 0x2020008C;

// Pin pull-up/pull-down enable
// const GPPUD: u32 = 0x20200094;
// Pin pull-up/pull-down enabe clock
// const GPPUDCLK0: u32 = 0x20200098;
// const GPPUDCLK1: u32 = 0x2020009C;


#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Pin {
    Tx = 14,
    Rx = 15,
    TwentyThree = 23,
    TwentyFour = 24,
}

#[allow(dead_code)]
pub enum Mode {
    Input = 0b000,
    Output = 0b001,
    Alt5 = 0b010,
}

pub fn write(pin: Pin, value: bool) {
    let gpio = GPIO_BASE as *const u32;
    let pin_num = pin as u32;

    if value {
        let led_on = unsafe { gpio.offset(8) as *mut u32 };
        unsafe { volatile_store(led_on, 1 << pin_num); }
    } else {
        let led_off = unsafe { gpio.offset(11) as *mut u32 };
        unsafe { volatile_store(led_off, 1 << pin_num); }
    }
}
pub fn read(pin: Pin) -> bool {
    let pin_num = pin as u32;
    let reg = if pin_num <= 31 {
        GPLEV0
    } else {
        GPLEV1
    } as *mut u32;

    system_memory_read_barrier();

    let mut val = unsafe { volatile_load(reg) };
    val = val >> (pin_num % 32);

    return (val & 1) != 0;
}

pub fn set_mode(pin: Pin, mode: Mode) {
    let gpio = GPIO_BASE as *const u32;

    let pin_num = pin as isize;
    let mode_val = mode as u32;

    unsafe {
        let gpio_pin_control = gpio.offset(pin_num / 10) as *mut u32;
        volatile_store(gpio_pin_control, volatile_load(gpio_pin_control) | mode_val << ((pin_num * 3) % 30));
    }
}

fn system_memory_read_barrier() {
    // DSB: data synchronization barrier
    unsafe { asm!("mcr p15, 0, $0, c7, c10, 4" : : "r" (0) : "memory" : "volatile"); }
}
fn system_memory_write_barrier() {
  // DSB: data synchronization barrier
  unsafe { asm!("mcr p15, 0, $0, c7, c10, 4" : : "r" (0) : "memory" : "volatile"); }
}

pub fn detect_falling_edge(pin: Pin) {
    let reg = if pin as u32 <= 31 {
        GPFEN0
    } else {
        GPFEN1
    } as *mut u32;
    let offset = (pin as u32) % 32;

    system_memory_read_barrier();

    let mut val = unsafe { volatile_load(reg) };
    val |= 1 << offset;
    unsafe { volatile_store(reg, val); }

    system_memory_write_barrier();
}


fn pin_to_event_register(pin: Pin) -> u32 {
    let pin_num = pin as u32;
    if pin_num <= 31 {
        return GPEDS0;
    } else {
        return GPEDS1;
    }
}
pub fn check_and_clear_event(pin: Pin) -> u32 {
    let reg = pin_to_event_register(pin) as *mut u32;
    let offset = (pin as u32) % 32;

    system_memory_read_barrier();

    let val: u32 = unsafe { volatile_load(reg) };
    let mask: u32 = 1 << offset;
    unsafe { volatile_store(reg, mask); } // Writing a 1 clears the bit

    system_memory_write_barrier();

    return val & mask;
}
