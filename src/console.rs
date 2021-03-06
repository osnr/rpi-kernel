use core::str;
use core::fmt;
use core::fmt::Write;

use uart;
use keyboard;
use gl;
use gl::font;
use reset;

use gpio;

pub struct Console {
    row: usize,
    col: usize,
}

impl Console {
    pub fn new() -> Console {
        Console { row: 0, col: 0 }
    }

    pub fn run(&mut self) {
        self.println("hello");

        loop {
            self.print("> ");

            let mut buf: [u8; 100] = [0; 100];

            self.readln(&mut buf);
            self.eval(unsafe { str::from_utf8_unchecked(&buf) });
        }
    }

    fn eval(&self, s: &str) {
        if s.starts_with("reset") {
            reset::reset();
        }
    }

    pub fn println(&mut self, s: &str) {
        self.print(s);
        self.print("\n");
    }
    fn print(&mut self, s: &str) {
        for c in s.bytes() {
            self.putchar(c);
        }
    }
    fn putchar(&mut self, c: u8) {
        if self.row * (font::HEIGHT + 1) >= gl::HEIGHT {
            gl::clear();
            self.row = 0;
            self.col = 0;
        }

        if c >= 32 && (c - 32) < 95 {
            gl::put_char(c, self.col * font::WIDTH, self.row * font::HEIGHT);

            self.col += 1;
            if self.col * font::WIDTH >= gl::WIDTH {
                self.row += 1;
                self.col = 0;
            }
        } else if c == '\n' as u8 {
            self.row += 1;
            self.col = 0;
        }
    }

    fn readln(&mut self, buf: &mut [u8; 100]) {
        let mut i: usize = 0;
        loop {
            let c = if uart::hasc() {
                uart::getc()
            } else if keyboard::has_char() {
                keyboard::read_char()
            } else {
                continue
            };

            uart::putc(c);

            match c as char {
                '\x08' => {
                    // Backspace.
                    if self.col >= 2 {
                        self.col -= 1;
                        self.putchar(' ' as u8);
                        self.col -= 1;

                        i -= 1;
                    } else {
                        self.col = 0;
                        i = 0;
                    }
                },
                '\n' => {
                    self.putchar(c);
                    return;
                },
                _ => {
                    self.putchar(c);

                    buf[i] = c;
                    i += 1;
                }
            }
        }
    }
}


impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.print(s);
        return Ok(());
    }
}
