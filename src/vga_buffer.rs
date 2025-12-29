use core::fmt;
use spin::Lazy;
use spin::Mutex;

pub static VGA_WRITER: Lazy<Mutex<VgaWriter>> = Lazy::new(|| {
    Mutex::new(VgaWriter::new(ColorCode::new(
        Color::LightBlue,
        Color::Black,
    )))
});

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
        ColorCode((background as u8) << 4 | (foreground as u8))
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
const VGA_BUFFER_ADDRESS: u32 = 0xb8000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaBuffer {
    pub chars: [[VgaCharacter; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VgaWriter {
    buffer: &'static mut VgaBuffer,
    column: usize,
    color_code: ColorCode,
}

#[allow(dead_code)]
impl VgaWriter {
    pub fn new(color_code: ColorCode) -> Self {
        Self {
            buffer: unsafe { &mut *(VGA_BUFFER_ADDRESS as *mut VgaBuffer) },
            color_code,
            column: 0,
        }
    }

    pub fn write_byte(&mut self, ascii_character: u8) {
        if ascii_character == b'\n' {
            self.new_line();
            return;
        }

        let row: usize = BUFFER_HEIGHT - 1;
        unsafe {
            core::ptr::write_volatile(
                &mut self.buffer.chars[row][self.column] as *mut VgaCharacter,
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
                        &mut self.buffer.chars[row][column] as *const VgaCharacter,
                    );
                    core::ptr::write_volatile(
                        &mut self.buffer.chars[row - 1][column],
                        character_to_move,
                    );
                }
            }
        }
        self.clear_last_line();
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
                    &mut self.buffer.chars[line_number][column],
                    VgaCharacter {
                        ascii_character: b' ',
                        character_color: self.color_code,
                    },
                );
            }
        }
    }

    pub fn clear(&mut self) {
        for row in (0..BUFFER_HEIGHT).into_iter().rev() {
            self.clear_line(row);
        }
    }
    pub fn clear_last_line(&mut self) {
        self.clear_line(BUFFER_HEIGHT - 1);
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::vga_buffer::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints the given formatted string to the VGA text buffer through the global `VGA_WRITER` instance.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGA_WRITER.lock().write_fmt(args).unwrap();
}
