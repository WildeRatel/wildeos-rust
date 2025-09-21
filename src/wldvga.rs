#![allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// 16 Bit colors.
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        color_code: ColorCode::new(Color::White, Color::Blue),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // In ascii range.
                0x20..0x7e | b'\n' => self.write_byte(byte),
                //Not in ascii range.
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        self.column_position = 0;
        self.row_position += 1;
        if (self.row_position) > BUFFER_HEIGHT - 1 {
            for i in 1..BUFFER_HEIGHT {
                for j in 0..BUFFER_WIDTH {
                    let temp = self.buffer.chars[i][j].read();
                    self.buffer.chars[i - 1][j].write(temp);
                }
            }
        }
        let blank_char = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for i in 0..BUFFER_WIDTH {
            self.buffer.chars[BUFFER_HEIGHT - 1][i].write(blank_char);
        }
    }

    // Fills the screen with a color.
    pub fn vga_paint(&mut self) {
        for i in 0..BUFFER_HEIGHT {
            for j in 0..BUFFER_WIDTH {
                let blank_char = ScreenChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                };

                self.buffer.chars[i][j].write(blank_char);
            }
        }
    }

    pub fn put_char(&mut self, character: u8, column: usize, row: usize) {
        let prev_col = self.column_position.clone();
        let prev_row = self.row_position.clone();
        self.column_position = column;
        self.row_position = row;
        self.write_byte(character);
        self.column_position = prev_col;
        self.row_position = prev_row;
    }
}

// Printing integers, floats, anything other than a char basically. Now we can use "write!"
use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[test_case]
fn check_buffer_range() {
    use crate::serial_print;
    use crate::serial_println;
    serial_print!("BUFFER_WIDTH:\t\t");
    assert_eq!(BUFFER_WIDTH, 80);
    serial_println!("[ok]");
    serial_print!("BUFFER_HEIGHT:\t\t");
    assert_eq!(BUFFER_HEIGHT, 25);
    serial_println!("[ok]");
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::wldvga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
