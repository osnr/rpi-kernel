const GPIO_BASE: u32 = 0x20200000;

pub enum Pin {
    Act = 15,
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
