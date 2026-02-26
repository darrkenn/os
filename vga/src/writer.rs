use core::fmt;

use crate::{
    BUFFER_HEIGHT, BUFFER_WIDTH, VGA_BUFFER,
    colour::{Colour, ColourCode},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct Char {
    ascii_char: u8,
    colour_code: ColourCode,
}

impl Default for Char {
    fn default() -> Self {
        Char {
            ascii_char: 0,
            colour_code: ColourCode::default(),
        }
    }
}

#[repr(transparent)]
struct Buffer {
    chars: [[volatile::Volatile<Char>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column: usize,
    row: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            column: 0,
            row: 0,
            colour_code: ColourCode::new(Colour::White, Colour::Black),
            buffer: unsafe { &mut *(VGA_BUFFER as *mut Buffer) },
        }
    }
    pub fn write(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
    pub fn set_colour_code(&mut self, colour_code: ColourCode) {
        self.colour_code = colour_code
    }
    pub fn location(&self) -> (usize, usize) {
        (self.row, self.column)
    }
    // Clear
    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col].write(Char::default())
            }
        }
        self.row = 0;
        self.column = 0;
    }
    pub fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row - 1][col].write(Char::default())
        }
    }
    pub fn clear_col(&mut self, col: usize) {
        for row in 0..BUFFER_HEIGHT {
            self.buffer.chars[row][col - 1].write(Char::default())
        }
    }
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = self.row;
                let col = self.column;

                let colour_code = self.colour_code;

                self.buffer.chars[row][col].write(Char {
                    ascii_char: byte,
                    colour_code,
                });

                self.column += 1;
            }
        }
    }

    pub fn new_line(&mut self) {
        self.column = 0 as usize;
        if self.row != BUFFER_HEIGHT - 1 {
            self.row += 1 as usize;
        } else {
            self.row = BUFFER_HEIGHT - 1;
            for row in 0..BUFFER_HEIGHT {
                if row == BUFFER_HEIGHT - 1 {
                    self.clear_row(BUFFER_HEIGHT);
                } else {
                    for col in 0..BUFFER_WIDTH {
                        let next_row_char = self.buffer.chars[row + 1][col].read();
                        self.buffer.chars[row][col].write(next_row_char);
                    }
                }
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}
