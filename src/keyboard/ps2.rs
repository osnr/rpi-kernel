use gpio;
use core::intrinsics::volatile_store;

use ringbuf;
use ringbuf::Buf;

const CLOCK: gpio::Pin = gpio::Pin::TwentyThree;
const DATA: gpio::Pin = gpio::Pin::TwentyFour;

const INTERRUPT_ENABLE_2: u32 = 0x2000b214;
const INTERRUPT_DISABLE_1: u32 = 0x2000b21c;
const INTERRUPT_DISABLE_2: u32 = 0x2000b220;

pub fn init() {
    unsafe {
        volatile_store(INTERRUPT_DISABLE_1 as *mut u32, 0xFFFFFFFF);
        volatile_store(INTERRUPT_DISABLE_2 as *mut u32, 0xFFFFFFFF);
    }

    gpio::set_mode(CLOCK, gpio::Mode::Input);
    gpio::set_mode(DATA, gpio::Mode::Input);

    gpio::detect_falling_edge(CLOCK);

    unsafe {
        volatile_store(INTERRUPT_ENABLE_2 as *mut u32, 1 << (52 - 32));
    }
}

pub static mut scanbuf: Buf<u8> = Buf {
    elems: [None; ringbuf::SIZE],
    head: 0,
    tail: 0,
};

enum Ps2 {
    AwaitingStart,
    Started,
    GotData { code: u8, pos: i32, parity: u8 },
    GotParity { code: u8 },
}

static mut scan: Ps2 = Ps2::AwaitingStart;

pub unsafe fn interrupt() {
    match scan {
        Ps2::AwaitingStart => {
            if !gpio::read(DATA) {
                // Start bit was low. Good.
                scan = Ps2::Started;
            } else {
                // Failure. Start bit was high.
                scan = Ps2::AwaitingStart;
            }
        },
        Ps2::Started => {
            let bit = gpio::read(DATA) as u8;
            scan = Ps2::GotData {
                code: bit,
                pos: 1,
                parity: bit,
            };
        },
        Ps2::GotData { code, pos, parity } => {
            let bit = gpio::read(DATA) as u8;
            if pos < 8 {
                scan = Ps2::GotData {
                    code: code | (bit << pos),
                    pos: pos + 1,
                    parity: parity + bit,
                };

            } else {
                // Handle parity bit.
                if (parity + bit) % 2 != 1 {
                    // Failure.
                    scan = Ps2::AwaitingStart;
                } else {
                    scan = Ps2::GotParity { code: code };
                }
            }
        },
        Ps2::GotParity { code } => {
            if gpio::read(DATA) {
                // Stop bit was high. Good.
                scanbuf.push(code);
                scan = Ps2::AwaitingStart;
            } else {
                scan = Ps2::AwaitingStart;
            }
        },
    }

    gpio::check_and_clear_event(CLOCK);
}

pub const SHIFT_SCAN_TABLE: [char; 128] = [
  ' ', // 0x00
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  ' ', // 0x08
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  '\t',  
  '~',  
  ' ',  
  ' ', // 0x10
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  'Q',
  '!',
  ' ',
  ' ', // 0x18
  ' ',
  'Z',
  'S',
  'A',
  'W',
  '@',
  ' ',
  ' ', // 0x20
  'C',
  'X',
  'D',
  'E',
  '$',
  '#',
  ' ',
  ' ', // 0x28
  ' ',
  'V',
  'F',
  'T',
  'R',
  '%',
  ' ',
  ' ', // 0x30
  'N',  
  'B',  
  'H',  
  'G',  
  'Y',  
  '^',  
  ' ',  
  ' ', // 0x38
  ' ',  
  'M',  
  'J',  
  'U',  
  '&',  
  '*',  
  ' ',  
  ' ', // 0x40
  '<',  
  'L',  
  'I',  
  'O',  
  ')',
  '(',
  ' ',
  ' ', // 0x48
  '>',
  '?',
  'L',
  ':',
  'P',
  '_',
  ' ',
  ' ', // 0x50
  ' ',
  '\"',
  ' ',
  '{',
  '+',
  ' ',
  ' ',
  ' ', // 0x58
  ' ',
  '\n',
  '}',
  ' ',
  '|',
  ' ', 
  ' ',
  ' ', // 0x60
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  ' ',
  '\x08',
  ' ',
  ' ', // 0x68
  '1',
  ' ',
  '4',
  '7',
  ' ',
  ' ',
  ' ',
  '0', // 0x70
  '.',
  '2',
  '5',
  '6',
  '8',
  ' ',
  ' ',
  ' ', // 0x78
  '+',
  '3',
  '-',
  '*',
  '9',
  ' ', 
  ' '
];

pub const SCAN_TABLE: [char; 128] = [
  ' ', // 0x00
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  ' ', // 0x08
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  '\t',  
  '`',  
  ' ',  
  ' ', // 0x10
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  'q',
  '1',
  ' ',
  ' ', // 0x18
  ' ',
  'z',
  's',
  'a',
  'w',
  '2',
  ' ',
  ' ', // 0x20
  'c',
  'x',
  'd',
  'e',
  '4',
  '3',
  ' ',
  ' ', // 0x28
  ' ',
  'v',
  'f',
  't',
  'r',
  '5',
  ' ',
  ' ', // 0x30
  'n',  
  'b',  
  'h',  
  'g',  
  'y',  
  '6',  
  ' ',  
  ' ', // 0x38
  ' ',  
  'm',  
  'j',  
  'u',  
  '7',  
  '8',  
  ' ',  
  ' ', // 0x40
  ',',  
  'k',  
  'i',  
  'o',  
  '0',
  '9',
  ' ',
  ' ', // 0x48
  '.',
  '/',
  'l',
  ';',
  'p',
  '-',
  ' ',
  ' ', // 0x50
  ' ',
  '\'',
  ' ',
  '[',
  '=',
  ' ',
  ' ',
  ' ', // 0x58
  ' ',
  '\n',
  ']',
  ' ',
  '\\',
  ' ', 
  ' ',
  ' ', // 0x60
  ' ',  
  ' ',  
  ' ',  
  ' ',  
  ' ',
  '\x08',
  ' ',
  ' ', // 0x68
  '1',
  ' ',
  '4',
  '7',
  ' ',
  ' ',
  ' ',
  '0', // 0x70
  '.',
  '2',
  '5',
  '6',
  '8',
  ' ',
  ' ',
  ' ', // 0x78
  '+',
  '3',
  '-',
  '*',
  '9',
  ' ', 
  ' '
];
