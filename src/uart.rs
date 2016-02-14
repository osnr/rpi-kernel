use gpio;
use core::intrinsics::volatile_load;

use core::fmt;

#[allow(dead_code)]
#[repr(C)]
pub struct Uart {
    data: u32, // I/O Data
    ier: u32,  // Interupt enable
    iir: u32,  // Interupt identify and fifo enables/clears
    lcr: u32,  // line control register
    mcr: u32,  // modem control register
    lsr: u32,  // line status
    msr: u32,  // line status
    scratch: u32,
    cntl: u32, // control register
    stat: u32, // status register
    baud: u32, // baud rate register
}

// AUX bits
const AUX_ENABLES: u32 = 0x20215004;
const AUX_MU_ENABLE: u32 = 0x00000001;
#[allow(dead_code)]
const AUX_SPI0_ENABLE: u32 = 0x00000002;
#[allow(dead_code)]
const AUX_SPI1_ENABLE: u32 = 0x00000004;

// Mini UART bits
const AUX_MU_UART: u32 = 0x20215040;

const AUX_MU_IIR_RX_FIFO_CLEAR: u32 = 0x00000002;
const AUX_MU_IIR_TX_FIFO_CLEAR: u32 = 0x00000004;
const AUX_MU_IIR_RX_FIFO_ENABLE: u32 = 0x00000008;
const AUX_MU_IIR_TX_FIFO_ENABLE: u32 = 0x00000004;

const AUX_MU_LCR_8BIT: u32 = 0x00000003;

const AUX_MU_LSR_RX_READY: u32 = 0x00000001;
#[allow(dead_code)]
const AUX_MU_LSR_TX_READY: u32 = 0x00000010;
const AUX_MU_LSR_TX_EMPTY: u32 = 0x00000020;

const AUX_MU_CNTL_TX_ENABLE: u32 = 0x00000002;
const AUX_MU_CNTL_RX_ENABLE: u32 = 0x00000001;

pub fn get_uart() -> &'static mut Uart {
    let uart = AUX_MU_UART as *mut Uart;
    let uart_ref = unsafe { &mut *uart };

    return uart_ref;
}

pub fn init() {
    let uart = get_uart();
    let aux = AUX_ENABLES as *mut u32;
    unsafe { *aux = AUX_MU_ENABLE; }

    uart.ier = 0;
    uart.cntl = 0;
    uart.lcr = AUX_MU_LCR_8BIT;
    uart.mcr = 0;
    uart.ier = 0;
    uart.iir = AUX_MU_IIR_RX_FIFO_CLEAR|
    AUX_MU_IIR_RX_FIFO_ENABLE|
    AUX_MU_IIR_TX_FIFO_CLEAR|
    AUX_MU_IIR_TX_FIFO_ENABLE;
    uart.baud = 270;

    gpio::set_mode(gpio::Pin::Tx, gpio::Mode::Alt5);
    gpio::set_mode(gpio::Pin::Rx, gpio::Mode::Alt5);

    uart.cntl = AUX_MU_CNTL_TX_ENABLE|AUX_MU_CNTL_RX_ENABLE;
}

pub fn hasc() -> bool {
    let lsr = &get_uart().lsr as *const u32;
    return unsafe { volatile_load(lsr) } & AUX_MU_LSR_RX_READY != 0;
}

pub fn getc() -> u8 {
    let uart = get_uart();
    while uart.lsr & AUX_MU_LSR_RX_READY == 0 {
        unsafe { asm!(""); }
    }

    return (uart.data & 0xFFu32) as u8;
}

pub fn putc(c: u8) {
    let uart = get_uart();
    if c == '\n' as u8 {
        putc('\r' as u8);
    }

    while uart.lsr & AUX_MU_LSR_TX_EMPTY == 0 {
        unsafe { asm!(""); }
    }

    uart.data = c as u32;
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.bytes() {
            putc(c);
        }
        return Ok(());
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (use core::fmt::Write; uart::get_uart().write_fmt(format_args!($($arg)*)).ok());
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
