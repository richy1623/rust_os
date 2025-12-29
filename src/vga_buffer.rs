use core::fmt;

/// The standard color palette in VGA text mode.
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
    Gray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// A combination of a VGA foreground and background color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        Self((foreground as u8) << 4 | (background as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VgaCharacter {
    pub ascii_character: u8,
    pub character_color: ColorCode,
}

/// The height of the text buffer (normally 25 lines).
const BUFFER_HEIGHT: usize = 25;
/// The width of the text buffer (normally 80 columns).
const BUFFER_WIDTH: usize = 80;
const VGA_BUFFER_ADDRESS: usize = 0xb8000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaBuffer {
    pub chars: [[VgaCharacter; BUFFER_HEIGHT]; BUFFER_WIDTH],
}

struct VgaWriter {
    buffer: VgaBuffer,
    column: usize,
    color_code: ColorCode,
}

impl VgaWriter {
    pub fn new(color_code: ColorCode) -> Self {
        Self {
            buffer: unsafe { core::ptr::read_volatile(VGA_BUFFER_ADDRESS as *mut VgaBuffer) },
            color_code,
            column: 0,
        }
    }

    pub fn write_byte(&mut self, ascii_character: u8) {
        if ascii_character == b'\n' {
            self.new_line();
        }

        let row: usize = BUFFER_HEIGHT - 1;
        unsafe {
            core::ptr::write_volatile(
                &mut self.buffer.chars[self.column][row],
                VgaCharacter {
                    ascii_character,
                    character_color: self.color_code,
                },
            );
        }
        self.column += 1;
    }

    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH {
                unsafe {
                    let character_to_move: VgaCharacter = core::ptr::read_volatile(
                        &mut self.buffer.chars[column][row] as *const VgaCharacter,
                    );
                    core::ptr::write_volatile(
                        &mut self.buffer.chars[column][row - 1],
                        character_to_move,
                    );
                }
            }
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn clear_line(&mut self, line_number: usize) {
        for column in 0..BUFFER_WIDTH {
            unsafe {
                core::ptr::write_volatile(
                    &mut self.buffer.chars[column][line_number],
                    VgaCharacter {
                        ascii_character: b' ',
                        character_color: self.color_code,
                    },
                );
            }
        }
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_line(row);
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// * Write byte
// * Write string
// * Write newline
