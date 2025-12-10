//! Provides abstraction for printing with VGA

use core::{
    fmt::write,
    ptr::{read_volatile, write_volatile},
};

const FOREGROUND_SIZE: u8 = 4;
const ASCII_CHAR_SIZE: u16 = 8;

const VGA_BUF_WIDTH: usize = 80;
const VGA_BUF_HEIGHT: usize = 24;

const VGA_ADDR: *mut u16 = 0xb8000 as *mut u16;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

type ColorCode = u8;

fn new_color_code(background: Color, foreground: Color) -> ColorCode {
    (background as u8) << FOREGROUND_SIZE | (foreground as u8)
}

fn default_color() -> ColorCode {
    new_color_code(Color::Black, Color::White)
}

type DisplayChar = u16;

fn new_display_char_color(ascii_char: u8, color_code: ColorCode) -> DisplayChar {
    (color_code as u16) << ASCII_CHAR_SIZE | (ascii_char as u16)
}

fn default_char(ascii_char: u8) -> DisplayChar {
    new_display_char_color(ascii_char, default_color())
}

pub struct Writer {
    color_code: ColorCode,
    col: usize,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            color_code: default_color(),
            col: 0,
        }
    }

    pub fn write(&mut self, msg: &str) {
        for ascii_char in msg.bytes() {
            match ascii_char {
                b'\n' => self.new_line(),
                _ => unsafe {
                    write_volatile(
                        VGA_ADDR.offset((self.col + VGA_BUF_HEIGHT * VGA_BUF_WIDTH) as isize),
                        new_display_char_color(ascii_char, self.color_code),
                    );
                    self.col += 1;
                },
            }
        }
    }

    fn new_line(&mut self) {
        // Copy rows "scroll" up
        for row in 1..=VGA_BUF_HEIGHT {
            for col in 0..=VGA_BUF_WIDTH {
                let old_pos = (row * VGA_BUF_WIDTH) + col;
                let new_pos = old_pos - VGA_BUF_WIDTH;

                unsafe {
                    let old_char = read_volatile(VGA_ADDR.offset(old_pos as isize));
                    write_volatile(VGA_ADDR.offset(new_pos as isize), old_char);
                }
            }
        }

        // Clear last row
        for col in 0..VGA_BUF_WIDTH {
            let pos = VGA_BUF_HEIGHT * VGA_BUF_WIDTH + col;
            unsafe {
                write_volatile(
                    VGA_ADDR.offset(pos as isize),
                    new_display_char_color(b' ', self.color_code),
                );
            }
        }
        self.col = 0;
    }
}
