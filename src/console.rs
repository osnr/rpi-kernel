use core::str;

use uart;
use keyboard;
use gl;
use gl::font;
use reset;

use gpio;

pub struct Console {
    pub row: usize,
    pub col: usize,
}

impl Console {
    pub fn run(&mut self) {
        self.println("hello");

        loop {
            self.print("> ");

            let mut buf: [u8; 100] = [0; 100];

            self.readln(&mut buf);
            self.eval(unsafe { str::from_utf8_unchecked(&buf) });

            gpio::write(gpio::Pin::Rx, true);
        }
    }

    fn eval(&self, s: &str) {
        if s.starts_with("reset") {
            reset::reset();
        }
    }

    fn println(&mut self, s: &str) {
        self.print(s);
        self.print("\n");
    }
    fn print(&mut self, s: &str) {
        for c in s.bytes() {
            self.putchar(c);
        }
    }
    fn putchar(&mut self, c: u8) {
        if c >= 32 {
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

        if self.row * font::HEIGHT >= gl::HEIGHT {
            gl::clear();
            self.row = 0;
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

            match c as char {
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
