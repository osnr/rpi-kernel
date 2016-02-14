const GPIO_BASE: u32 = 0x20200000;

#[allow(dead_code)]
pub enum Pin {
    Tx = 14,
    Rx = 15,
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
        unsafe { *(led_on) = 1 << pin_num; }
    } else {
        let led_off = unsafe { gpio.offset(11) as *mut u32 };
        unsafe { *(led_off) = 1 << pin_num; }
    }
}

pub fn set_mode(pin: Pin, mode: Mode) {
    let gpio = GPIO_BASE as *const u32;

    let pin_num = pin as isize;
    let mode_val = mode as u32;

    unsafe {
        let gpio_pin_control = gpio.offset(pin_num / 10) as *mut u32;
        *(gpio_pin_control) |= mode_val << ((pin_num * 3) % 30);
    }
}
